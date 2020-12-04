use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

pub const SOUL: &str = "#";
pub const METADATA: &str = "_";
pub const STATE: &str = ">";

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
