use std::{
	collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use serde_json::Value;
use tokio::{net::{TcpListener, TcpStream}};
use tungstenite::protocol::Message;

use crate::dedup::Dedup;

struct Store {
	peers: HashMap<SocketAddr, UnboundedSender<Message>>,
	dedup: Dedup,
	count: u32,
}

impl Store {
	pub fn new() -> Self {
		Self {
			peers: HashMap::new(),
			dedup: Dedup::new(),
			count: 0,
		}
	}
}

async fn handle_connection(store: Arc<Mutex<Store>>, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    store.lock().unwrap().peers.insert(addr, tx.clone());

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
    	let msg_str = msg.to_text().unwrap();
    	let msg: Value = serde_json::from_str(msg_str).expect("Could not parse JSON");

        let id = msg["#"]
        	.as_str()
        	.expect("ID must be a string")
        	.to_owned();

        if store.lock().unwrap().dedup.check(id.clone()).is_none() {
        	store.lock().unwrap().dedup.track(id);
        	println!("recieved: {:?}", msg);

            for tx in store.lock().unwrap().peers.values() {
                tx.unbounded_send(msg_str.into()).unwrap();
            }
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    store.lock().unwrap().peers.remove(&addr);
}

pub async fn boot_socket() -> Result<()> {
    let addr = "127.0.0.1:8080".to_owned();
    let store = Arc::new(Mutex::new(Store::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(store.clone(), stream, addr));
    }

    Ok(())
}
