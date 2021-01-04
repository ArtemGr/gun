use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_std::task;
use async_trait::async_trait;
use serde_json::{json, Value as JSON};

use gun::{
	dedup::{Dedup, random_soul},
	GunBuilder,
    GunOptions,
    plugins::GunPlugin,
	ham::mix_ham,
	util::{lex_from_graph, parse_json, METADATA, SOUL},
};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Store {
	dedup: Dedup,
    graph: JSON,
    last_msg: Option<String>,
}

impl Store {
	pub fn new() -> Self {
		Self {
			dedup: Dedup::new(),
            graph: json!({}),
            last_msg: None,
		}
	}
}

pub struct WebsocketsWASM {
    store: Arc<Mutex<Store>>,
}

impl WebsocketsWASM {
	pub fn new() -> Self {
		Self {
			store: Arc::new(Mutex::new(Store::new()))
		}
	}

	pub fn plug_into(gun: &mut GunBuilder) {
		gun.plugin = Arc::new(Box::new(WebsocketsWASM::new()));
	}
}

fn handle_message(store: &Arc<Mutex<Store>>, msg_str: &str) {
    store.lock().unwrap().last_msg = Some(msg_str.into());

    match parse_json(msg_str) {
        Some(msg) => {
            let soul = match msg[SOUL].as_str() {
                Some(soul) => soul,
                None => return,
            };

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
                            // let soul = store.lock().unwrap().dedup.track(random_soul());

                            // let data = json!({
                            //     SOUL: soul.as_str(),
                            //     "@": msg[SOUL],
                            //     "put": ack,
                            // }).to_string();

                            // emit(
                            //     &store.lock().unwrap().peers,
                            //     data,
                            // );

                            log::info!("GET {}", ack);
                        },
                        Err(err) => log::error!("{}", err),
                    }
                }
                
                // emit(&store.lock().unwrap().peers, msg_str.into());
            }
        },
        None => (),
    }
}

async fn handle_connection(store: Arc<Mutex<Store>>, ws: WebSocket) {
	ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(msg) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let msg: String = msg.to_string().into();
            handle_message(&store, &msg);
        // } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
        } else if let Ok(msg) = e.data().dyn_into::<js_sys::JsString>() {
        	let msg: String = msg.into();
            handle_message(&store, &msg);
        } else {
            console_log!("Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}

fn replace_http(url: &str) -> String {
    if url.contains("https") {
        url.replace("https", "wss").into()
    } else if url.contains("http") {
        url.replace("http", "wss").into()
    } else {
        url.into()
    }
}

#[async_trait]
impl GunPlugin for WebsocketsWASM {
	async fn start<'a>(&self, options: &GunOptions<'a>) -> Result<()> {
		for peer in options.peers {
            let url = replace_http(peer);
    		let ws = WebSocket::new(&url).unwrap();
            task::spawn_local(handle_connection(self.store.clone(), ws));
        }

		Ok(())
	}

	fn emit(&self, _data: String) {}

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

	fn wait_for_connection(&self) {}

	fn graph(&self) -> JSON {
		self.store.lock().unwrap().graph.clone()
	}
}
