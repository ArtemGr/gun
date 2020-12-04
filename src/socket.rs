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

use crate::dedup::Dedup;
use crate::ham::mix_ham;
use crate::util::parse_json;

struct Store {
	peers: HashMap<SocketAddr, UnboundedSender<Message>>,
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
                let id = msg["#"]
                    .as_str()
                    .expect("ID must be a string")
                    .to_owned();

                if store.lock().unwrap().dedup.check(id.clone()).is_none() {
                    store.lock().unwrap().dedup.track(id);

                    if msg.get("put").is_some() {
                        mix_ham(msg["put"].clone(), &mut store.lock().unwrap().graph);
                        log::info!("{}: {}", addr, store.lock().unwrap().graph);
                    }

                    for (addr, tx) in &store.lock().unwrap().peers {
                        match tx.unbounded_send(msg_str.into()) {
                            Ok(_) => (),
                            Err(err) => log::error!("{}: {}", addr, err),
                        }
                    }
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
