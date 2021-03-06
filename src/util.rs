#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use anyhow::{anyhow, Result};
use serde_json::{json, Value as JSON};

// #[cfg(feature = "default-uuid")]
// use uuid::Uuid;

use crate::plugins::GunPlugin;

#[cfg(feature = "std")]
pub type Plugin<'a> = Arc<Box<dyn GunPlugin + Send + Sync + 'a>>;

pub const SOUL: &str = "#";
pub const METADATA: &str = "_";
pub const STATE: &str = ">";
pub const ACK: &str = "@";

pub fn lex_from_graph(lex: JSON, graph: &JSON) -> Result<JSON> {
    let soul = match lex.get(SOUL) {
        Some(soul) => match soul.as_str() {
            Some(soul) => soul,
            None => {
                return Err(anyhow!("Soul must be a string"));
            },
        },
        None => {
            return Err(anyhow!("Soul ('#') property is missing from lex"));
        },
    };

    if let Some(node) = graph.get(soul) {
        let key = lex.get(".");
        
        let node = if let Some(key) = key {
            let key = match key.as_str() {
                Some(key) => key,
                None => {
                    return Err(anyhow!("Key must be a string"));
                },
            };

            if !node[key].is_null() {
                let metadata = json!({STATE: {key: node[METADATA][STATE][key].clone()}});
                
                json!({METADATA: metadata, key: node[key].clone()})
            } else {
                return Err(anyhow!("Could not find key: {}", key));
            }
        } else {
            node.clone()
        };

        Ok(json!({soul: node}))
    } else {
        Err(anyhow!("Could not find node: {}", soul))
    }
}

pub fn next_state(soul: &str, data: &JSON, graph: JSON) -> JSON {
    let mut state = json!({});

    if let Some(node) = graph.get(soul) {
        let node_state = &node[METADATA][STATE];

        for (key, val) in data.as_object().unwrap().iter() {
            state[key] = json!(match node_state.get(key) {
                Some(n) => {
                    let n = n.as_i64().unwrap();
                    log::info!("{:?}", node[key]);
                    log::info!("{:?}", val);
                    if &node[key] == val {
                        n
                    } else {
                        n + 1
                    }
                },
                None => 1,
            });
        }
    } else {
        for key in data.as_object().unwrap().keys() {
            state[key] = json!(1);
        }
    }

    state
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
export function timestamp() {
    return performance.now();
}"#)]
extern "C" {
    pub fn timestamp() -> f64;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Impossible! Time went backwards!")
        .as_millis();

    millis as f64 / 1000.0
}

pub fn parse_json(json: &str) -> Option<JSON> {
	match serde_json::from_str(json) {
		Ok(json) => Some(json),
		Err(err) => {
			log::error!("{}", err);
			None
		},
	}
}

// #[cfg(feature = "default-uuid")]
// #[cfg(not(target_arch = "wasm32"))]
// pub fn uuid() -> String {
//     Uuid::new_v4().to_string()
// }
