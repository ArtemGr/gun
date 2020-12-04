use serde_json::{json, Value};

use crate::util::{SOUL, METADATA, STATE};

pub fn get(lex: Value, graph: &Value) -> Option<Value> {
	let soul = lex[SOUL].as_str().unwrap();
	let key = &lex["."];
	let mut node = graph[soul].clone();

	if node.is_null() {
		return None
	}

	if !key.is_null() {
		let key = key.as_str().unwrap();
		let tmp = node[key].clone();

		if !tmp.is_null() {
			node = json!({METADATA: node[METADATA], key: tmp.clone()});
        	let tmp = node[METADATA][STATE].clone();
	        node[METADATA][STATE] = json!({key: tmp[key].clone()});
		} else {
			return None
		}
	}

	Some(json!({soul: node}))
}
