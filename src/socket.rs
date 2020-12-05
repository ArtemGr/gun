use std::{
	collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use serde_json::{json, Value};
use tokio::{net::{TcpListener, TcpStream}};
use tungstenite::protocol::Message;

use crate::dedup::{random_soul, Dedup};
use crate::ham::mix_ham;
use crate::util::{lex_from_graph, parse_json, SOUL};

type PeerList = HashMap<SocketAddr, UnboundedSender<Message>>;

struct Store {
	peers: PeerList,
	dedup: Dedup,
    graph: Value,
}

impl Store {
	pub fn new() -> Self {
		Self {
			peers: HashMap::new(),
			dedup: Dedup::new(),
            graph: json!({}),
		}
	}
}

fn emit(peers: &PeerList, msg: Message) {
    for (addr, tx) in peers {
        match tx.unbounded_send(msg.clone()) {
            Ok(_) => (),
            Err(err) => log::error!("{}: {}", addr, err),
        }
    }
}

async fn handle_connection(store: Arc<Mutex<Store>>, raw_stream: TcpStream, addr: SocketAddr) {
    log::info!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    log::info!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    store.lock().unwrap().peers.insert(addr, tx.clone());

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
    	let msg_str = msg.to_text().unwrap();

        match parse_json(msg_str) {
            Some(msg) => {
                let soul = msg[SOUL]
                    .as_str()
                    .expect("Soul must be a string")
                    .to_owned();

                if store.lock().unwrap().dedup.check(soul.clone()).is_none() {
                    store.lock().unwrap().dedup.track(soul);

                    if !msg["put"].is_null() {
                        mix_ham(msg["put"].clone(), &mut store.lock().unwrap().graph);
                        log::info!("{}: PUT {}", addr, store.lock().unwrap().graph);
                    }

                    if !msg["get"].is_null() {
                        let ack = lex_from_graph(msg["get"].clone(), &store.lock().unwrap().graph);

                        match ack {
                            Ok(ack) => {
                                let data = json!({
                                    SOUL: store.lock().unwrap().dedup.track(random_soul()),
                                    "@": msg[SOUL],
                                    "put": ack,
                                }).to_string();

                                emit(
                                    &store.lock().unwrap().peers,
                                    data.into(),
                                );

                                log::info!("{}: GET {}", addr, ack);
                            },
                            Err(err) => log::error!("{}", err),
                        }
                    }
                    
                    emit(&store.lock().unwrap().peers, msg_str.into());                    
                }
            },
            None => (),
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    log::info!("{} disconnected", &addr);
    store.lock().unwrap().peers.remove(&addr);
}

pub async fn boot_socket() -> Result<()> {
    let addr = "127.0.0.1:8080".to_owned();
    let store = Arc::new(Mutex::new(Store::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    log::info!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(store.clone(), stream, addr));
    }

    Ok(())
}
