use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use serde_json::{json, Value as JSON};
use uuid::Uuid;

pub const SOUL: &str = "#";
pub const METADATA: &str = "_";
pub const STATE: &str = ">";

pub type Str = smallstr::SmallString<[u8; 16]>;

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

#[cfg(target_arch = "wasm32")]
pub fn timestamp() -> f64 {
    unimplemented!()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp() -> f64 {
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

pub fn uuid() -> Str {
    Str::from_buf(*Uuid::new_v4().as_bytes()).unwrap()
}
