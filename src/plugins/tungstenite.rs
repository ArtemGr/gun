use std::{
	net::{TcpListener, TcpStream},
	process,
	str::from_utf8,
	sync::{Arc, Mutex},
	thread,
};

use anyhow::Result;
use serde_json::{json, Value as JSON};
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, WebSocket};

use crate::{
	dedup::{Dedup, random_soul},
	GunBuilder,
	GunFunctions,
	ham::mix_ham,
	util::{lex_from_graph, parse_json, SOUL},
};

type PeerList = Vec<Arc<Mutex<WebSocket<TcpStream>>>>;

struct Store {
	peers: PeerList,
	dedup: Dedup,
    graph: JSON,
}

impl Store {
	pub fn new() -> Self {
		Self {
			peers: Vec::new(),
			dedup: Dedup::new(),
            graph: json!({}),
		}
	}
}

fn emit<'a>(peers: &PeerList, msg: &str) {
    for socket in peers {
        match socket.try_lock() {
        	Ok(mut socket) => socket
        		.write_message(Message::Text(msg.into()))
        		.unwrap_or_else(|err| log::warn!("{}", err)),
        	Err(_) => (),
        }
    }
}

fn handle_message(store: &Arc<Mutex<Store>>, msg_str: &str) {
    match parse_json(msg_str) {
        Some(msg) => {
            let soul = msg[SOUL]
                .as_str()
                .expect("Soul must be a string");

            if store.lock().unwrap().dedup.check(soul.into()).is_none() {
                store.lock().unwrap().dedup.track(soul.into());

                if !msg["put"].is_null() {
                    mix_ham(msg["put"].clone(), &mut store.lock().unwrap().graph);
                    log::info!("PUT {}", store.lock().unwrap().graph);
                }

                if !msg["get"].is_null() {
                    let ack = lex_from_graph(msg["get"].clone(), &store.lock().unwrap().graph);

                    match ack {
                        Ok(ack) => {
                            let soul = store.lock().unwrap().dedup.track(random_soul());

                            let data = json!({
                                SOUL: soul.as_str(),
                                "@": msg[SOUL],
                                "put": ack,
                            }).to_string();

                            emit(
                                &store.lock().unwrap().peers,
                                data.as_str(),
                            );

                            log::info!("GET {}", ack);
                        },
                        Err(err) => log::error!("{}", err),
                    }
                }
                
                emit(&store.lock().unwrap().peers, msg_str);
            }
        },
        None => (),
    }
}

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => {
        	log::error!("Blocking socket would block");
        	process::exit(0)
        },
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(store: Arc<Mutex<Store>>, stream: TcpStream) -> tungstenite::Result<()> {
    let socket = accept(stream).map_err(must_not_block)?;
    let socket = Arc::new(Mutex::new(socket));
	store.lock().unwrap().peers.push(socket.clone());

    loop {
    	let mut socket = socket.lock().unwrap();
        match socket.read_message()? {
            Message::Text(msg) => {
            	drop(socket);
            	handle_message(&store, &msg)
            },
            Message::Binary(msg) => {
            	drop(socket);
            	handle_message(&store, from_utf8(msg.as_slice()).unwrap())
            },
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) => (),
        }
    }
}

pub fn start(peers: &[&str]) -> Result<()> {
	let server = TcpListener::bind("127.0.0.1:8080").unwrap();
	let store = Arc::new(Mutex::new(Store::new()));

	log::info!("Running on 127.0.0.1:8080");

    for stream in server.incoming() {
    	let store = store.clone();
        thread::spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(store, stream) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => log::error!("{}", e),
                    }
                }
            },
            Err(e) => log::error!("Error accepting stream: {}", e),
        });
    }

	Ok(())
}

pub fn plug_into(gun: &mut GunBuilder) {
	gun.functions = GunFunctions {
		start,
	}
}
