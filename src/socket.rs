use std::{
	collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Result;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use serde_json::{json, Value};
use tokio::{net::{TcpListener, TcpStream}, time};
use tungstenite::protocol::Message;

use crate::dup::Dup;

struct Store {
	peers: HashMap<SocketAddr, UnboundedSender<Message>>,
	dup: Dup,
	count: u32,
}

impl Store {
	pub fn new() -> Self {
		Self {
			peers: HashMap::new(),
			dup: Dup::new(),
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

    // Begin temporary

	let mut interval = time::interval(Duration::from_millis(1000));
	let store_clone = store.clone();

    tokio::spawn(async move {
    	while let _ = interval.tick().await {
    		store_clone.lock().unwrap().count += 1;

    		let id = store_clone.lock().unwrap().count.to_string();
    		let id = store_clone.lock().unwrap().dup.track(id);

    		let msg = json!({
    			"#": id,
    		});

    		tx.unbounded_send(msg.to_string().into()).unwrap();
	    }
    });

    // End temporary

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
    	let msg = msg.to_text().unwrap();
    	let msg: Value = serde_json::from_str(msg).expect("Could not parse JSON");

        let id = msg["#"]
        	.as_str()
        	.expect("ID must be a string")
        	.to_owned();

        let dup = &mut store.lock().unwrap().dup;

        // if dup.check(id.clone()).is_none() {
        	dup.track(id);
        	println!("recieved: {:?}", msg);
        // }

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
