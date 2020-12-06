use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use serde_json::{json, Value as JSON};
use uuid::Uuid;

pub const SOUL: &str = "#";
pub const METADATA: &str = "_";
pub const STATE: &str = ">";

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

pub fn timestamp() -> u64 {
	SystemTime::now()
    	.duration_since(UNIX_EPOCH)
    	.expect("Impossible! Time went backwards!")
    	.as_secs()
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

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
}
