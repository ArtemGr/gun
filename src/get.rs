use serde_json::{json, Value};

use crate::util::{SOUL, METADATA, STATE};

pub fn lex_from_graph(lex: Value, graph: &Value) -> Option<Value> {
	let soul = lex[SOUL].as_str().unwrap();

	if let Some(node) = graph.get(soul) {
		let key = lex.get(".");
		
		let node = if let Some(key) = key {
			let key = key.as_str().unwrap();

			if !node[key].is_null() {
				let metadata = json!({STATE: {key: node[METADATA][STATE][key].clone()}});
				
				json!({METADATA: metadata, key: node[key].clone()})
			} else {
				return None;
			}
		} else {
			return None;
		};

		Some(json!({soul: node}))
	} else {
		None
	}
}
