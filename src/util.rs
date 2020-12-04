use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

pub fn timestamp() -> u64 {
	SystemTime::now()
    	.duration_since(UNIX_EPOCH)
    	.expect("Impossible! Time went backwards!")
    	.as_secs()
}

pub fn parse_json(json: &str) -> Option<Value> {
	match serde_json::from_str(json) {
		Ok(value) => Some(value),
		Err(err) => {
			log::error!("{}", err);
			None
		},
	}
}
