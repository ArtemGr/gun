use std::{
	sync::{Arc, Mutex},
	time::{SystemTime, UNIX_EPOCH},
};

use fnv::FnvHashMap;
use tokio::time::{sleep, Duration};

const MAX: u64 = 1000;
const AGE: u64 = 1000 * 9;

fn now() -> u64 {
	SystemTime::now()
    	.duration_since(UNIX_EPOCH)
    	.expect("Impossible! Time went backwards!")
    	.as_secs()
}

pub struct Dup {
	timeline: Arc<Mutex<FnvHashMap<String, u64>>>,
	timeout: Arc<Mutex<bool>>,
}

impl Dup {
	pub fn new() -> Self {
		Self {
			timeline: Arc::new(Mutex::new(FnvHashMap::default())),
			timeout: Arc::new(Mutex::new(false)),
		}
	}

	pub fn check(&mut self, id: String) -> Option<String> {
		if self.timeline.lock().unwrap().contains_key(&id) {
			Some(self.track(id))
		} else {
			None
		}
	}

	pub fn track(&mut self, id: String) -> String {
		self.timeline.lock().unwrap().insert(id.clone(), now());

		if !*self.timeout.lock().unwrap() {
			*self.timeout.lock().unwrap() = true;

			let timeline = self.timeline.clone();
			let timeout = self.timeout.clone();

			tokio::spawn(async move {
				sleep(Duration::from_millis(1000)).await;

				for (id, time) in &*timeline.lock().unwrap() {
					if AGE > now() - time {
						break;
					}

					timeline.lock().unwrap().remove(id);
				}

				*timeout.lock().unwrap() = false;
			});
		}

		id
	}
}
