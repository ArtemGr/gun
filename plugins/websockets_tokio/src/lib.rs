use std::{
    collections::HashMap,
	sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use async_trait::async_trait;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, TryStreamExt, StreamExt};
use serde_json::{json, Value as JSON};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{connect_async, WebSocketStream};
use tungstenite::protocol::Message;
use url::Url;

use gun::{
	dedup::{Dedup, random_soul},
	GunBuilder,
    GunOptions,
    plugins::GunPlugin,
	ham::mix_ham,
	util::{lex_from_graph, parse_json, METADATA, SOUL},
};

type Tx = UnboundedSender<Message>;
type PeerList = HashMap<String, Tx>;

struct Store {
	peers: PeerList,
	dedup: Dedup,
    graph: JSON,
    last_msg: Option<String>,
}

impl Store {
	pub fn new() -> Self {
		Self {
			peers: HashMap::new(),
			dedup: Dedup::new(),
            graph: json!({}),
            last_msg: None,
		}
	}
}

pub struct WebsocketsTokio {
    store: Arc<Mutex<Store>>,
}

impl WebsocketsTokio {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(Store::new())),
        }
    }

    pub fn plug_into(gun: &mut GunBuilder) {
        gun.plugin = Arc::new(Box::new(Self::new()));
    }
}
    
fn emit(peers: &PeerList, msg: String) {
    for tx in peers.values() {
        tx.unbounded_send(Message::Text(msg.clone()))
            .unwrap_or_else(|err| log::error!("{}", err));
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

async fn handle_connection(store: Arc<Mutex<Store>>, stream: WebSocketStream<TcpStream>, addr: String) {
    let (tx, rx) = unbounded();
    store.lock().unwrap().peers.insert(addr.clone(), tx);

    let (outgoing, incoming) = stream.split();

    let incoming = incoming.try_for_each(|msg| {
        handle_message(&store, msg.to_text().unwrap());
        future::ok(())
    });

    let outgoing = rx.map(Ok).forward(outgoing);

    pin_mut!(incoming, outgoing);
    future::select(incoming, outgoing).await;

    store.lock().unwrap().peers.remove(&addr);
}

fn replace_http(url: &str) -> String {
    if url.contains("https") {
        url.replace("https", "ws").into()
    } else if url.contains("http") {
        url.replace("https", "ws").into()
    } else {
        url.into()
    }
}

#[async_trait]
impl GunPlugin for WebsocketsTokio {
    async fn start<'a>(&self, options: &GunOptions<'a>) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", options.port)).await?;

        log::info!("Running on 127.0.0.1:{}", options.port);

        for peer in options.peers {
            let url = replace_http(peer);
            let url = Url::parse(&url)?;
            let (stream, _) = connect_async(url).await?;
            tokio::spawn(handle_connection(self.store.clone(), stream, (*peer).into()));
        }

        while let Ok((stream, addr)) = listener.accept().await {
            let stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Error during the websocket handshake occurred");
            let addr = addr.to_string();
            tokio::spawn(handle_connection(self.store.clone(), stream, addr));
        }

        Ok(())
    }

    fn emit(&self, data: String) {
        emit(&self.store.lock().unwrap().peers, data.clone());
    }

    fn check(&self, key: &str) -> Option<JSON> {
        if let Some(data) = &self.store.lock().unwrap().last_msg {
            let data = parse_json(data).unwrap();
            if data["put"][key].is_null() { return None }
            let mut data = data["put"][key].clone();
            data[METADATA].take();
            return Some(data);
        }
        None
    }

    fn wait_for_connection(&self) {
        while self.store.lock().unwrap().peers.is_empty() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn graph(&self) -> JSON {
        self.store.lock().unwrap().graph.clone()
    }
}
