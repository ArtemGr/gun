use anyhow::{anyhow, Result};
use serde_json::{json, Value};

use crate::util::{SOUL, METADATA, STATE};

pub fn lex_from_graph(lex: Value, graph: &Value) -> Result<Value> {
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
