use std::{
	net::{TcpListener, TcpStream},
	process,
    rc::Rc,
	str::from_utf8,
	sync::{Arc, Mutex},
	thread::{self, JoinHandle},
};

use anyhow::{anyhow, Result};
use serde_json::{json, Value as JSON};
use tungstenite::{accept, connect, handshake::HandshakeRole, Error, HandshakeError, Message, WebSocket};
use url::Url;

use crate::{
	dedup::{Dedup, random_soul},
	GunBuilder,
    GunOptions,
    GunPlugin,
	ham::mix_ham,
	util::{lex_from_graph, parse_json, timestamp, SOUL},
};

type PeerList = Vec<Arc<Mutex<WebSocket<TcpStream>>>>;

struct Store {
	peers: PeerList,
	dedup: Dedup,
    graph: JSON,
    last_msg: Option<String>,
}

impl Store {
	pub fn new() -> Self {
		Self {
			peers: Vec::new(),
			dedup: Dedup::new(),
            graph: json!({}),
            last_msg: None,
		}
	}
}

pub struct Tungstenite {
    store: Arc<Mutex<Store>>,
    handle: Option<JoinHandle<()>>,
}

impl Tungstenite {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(Store::new())),
            handle: None,
        }
    }
}
    
fn emit<'a>(peers: &PeerList, msg: String) {
    for socket in peers {
        match socket.try_lock() {
            Ok(mut socket) => socket
                .write_message(Message::Text(msg.clone()))
                .unwrap_or_else(|err| log::warn!("{}", err)),
            Err(_) => (),
        }
    }
}

fn handle_message(store: &Arc<Mutex<Store>>, msg_str: &str) {
    store.lock().unwrap().last_msg = Some(msg_str.into());

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
                                data,
                            );

                            log::info!("GET {}", ack);
                        },
                        Err(err) => log::error!("{}", err),
                    }
                }
                
                emit(&store.lock().unwrap().peers, msg_str.into());
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
        store.lock().unwrap().last_msg = None;
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

impl GunPlugin for Tungstenite {
    fn start(&self, options: &GunOptions) -> Result<()> {
        let server = TcpListener::bind(format!("127.0.0.1:{}", options.port))?;

        log::info!("Running on 127.0.0.1:{}", options.port);

        for peer in options.peers {
            connect(Url::parse(peer).unwrap())?;
        }

        let store = self.store.clone();
        thread::spawn(move || {
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
        });

        Ok(())
    }

    fn emit(&self, data: String) {
        emit(&self.store.lock().unwrap().peers, data);
    }

    fn wait_for_data(&self, timeout: f64) -> Result<String> {
        let begin = timestamp();
        loop {
            if let Some(data) = &self.store.lock().unwrap().last_msg {
                return Ok(data.into());
            }
            if timestamp() - begin > timeout {
                return Err(anyhow!("Data request timed out"));
            }
        }
    }
}

pub fn plug_into(gun: &mut GunBuilder) {
    gun.plugin = Rc::new(Tungstenite::new());
}
