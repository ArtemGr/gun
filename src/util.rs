use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
	SystemTime::now()
    	.duration_since(UNIX_EPOCH)
    	.expect("Impossible! Time went backwards!")
    	.as_secs()
}
