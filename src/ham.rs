use anyhow::{anyhow, Result};
use serde_json::{json, Map, Value};

use crate::util::timestamp;

struct HAM {
	converge: bool,
	current: bool,
	defer: bool,
	historical: bool,
	incoming: bool,
	state: bool,
}

impl Default for HAM {
	fn default() -> Self {
		Self {
			converge: false,
			current: false,
			defer: false,
			historical: false,
			incoming: false,
			state: false,
		}
	}
}

fn ham(
	machine_state: u64,
	incoming_state: u64,
	current_state: u64,
	incoming_value: String,
	current_value: String,
) -> Result<HAM> {
	if machine_state < incoming_state {
        return Ok(HAM { defer: true, ..Default::default() });
    } else if incoming_state < current_state {
        return Ok(HAM { historical: true, ..Default::default() });
    } else if current_state < incoming_state {
        return Ok(HAM { converge: true, incoming: true, ..Default::default() });
    } else if incoming_state == current_state {
        if incoming_value == current_value {
        	return Ok(HAM { state: true, ..Default::default() });
        } else if incoming_value < current_value {
            return Ok(HAM { converge: true, current: true, ..Default::default() });
        } else if current_value < incoming_value {
            return Ok(HAM { converge: true, incoming: true, ..Default::default() });
        }
    }

    Err(anyhow!("Invalid CRDT Data: {} to {} at {} to {}!",
    	incoming_value, current_value, incoming_state, current_state))
}

pub fn mix_ham(change: Value, graph: &mut Value) -> Value {
	let machine = timestamp();
	let mut diff = Map::new();

	for (soul, node) in change.as_object().unwrap().iter() {
		for (key, val) in node.as_object().unwrap().iter() {
			if key == "_" {
				break;
			}

			let state = node["_"][">"][key].as_u64().unwrap();
			let was = match graph.get(soul) {
				Some(node) => match node["_"][">"].get(key) {
					Some(val) => val.as_u64().unwrap(),
					None => 0,
				},
				None => 0,
			};
			let val = val.to_string();
			let known = match graph.get(soul) {
				Some(node) => node[key].to_string(),
				None => "".to_string(),
			};

			let ham = match ham(machine, state, was, val.clone(), known) {
				Ok(ham) => ham,
				Err(err) => {
					log::error!("{}", err);
					break;
				},
			};

			if !ham.incoming {
				if ham.defer {
					log::info!("DEFER {} {}", key, val);
				}
				break;
			}

			if diff.get(soul).is_none() {
				diff.insert(soul.into(), json!({"_":{"#":soul, ">":{}}}));
			}

			if graph.get(soul).is_none() {
				graph.as_object_mut().unwrap().insert(soul.into(), json!({"_":{"#":soul, ">":{}}}));
			}

			graph.get_mut(soul).unwrap().as_object_mut().unwrap().insert(key.into(), json!(val));
			diff.get_mut(soul).unwrap().as_object_mut().unwrap().insert(key.into(), json!(val));

			graph.get_mut(soul).unwrap().get_mut("_").unwrap().get_mut(">").unwrap().as_object_mut().unwrap().insert(key.into(), json!(state));
			diff.get_mut(soul).unwrap().get_mut("_").unwrap().get_mut(">").unwrap().as_object_mut().unwrap().insert(key.into(), json!(state));
   		}
	}

	Value::Object(diff)
}
