use std::{
	sync::{Arc, Mutex},
};

use fnv::FnvHashMap;
use rand::prelude::*;
use substring::Substring;
use tokio::time::{sleep, Duration};

use crate::util::{format_radix, timestamp};

const MAX: u64 = 1000;
const AGE: u64 = 1000 * 9;

pub struct Dedup {
	timeline: Arc<Mutex<FnvHashMap<String, u64>>>,
	timeout: Arc<Mutex<bool>>,
}

impl Dedup {
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
		self.timeline.lock().unwrap().insert(id.clone(), timestamp());

		if !*self.timeout.lock().unwrap() {
			*self.timeout.lock().unwrap() = true;

			let timeline = self.timeline.clone();
			let timeout = self.timeout.clone();

			tokio::spawn(async move {
				sleep(Duration::from_millis(1000)).await;

				for (id, time) in &*timeline.lock().unwrap() {
					if AGE > timestamp() - time {
						continue;
					}

					timeline.lock().unwrap().remove(id);
				}

				*timeout.lock().unwrap() = false;
			});
		}

		id
	}
}

pub fn random_id() -> String {
	let id = rand::thread_rng().gen_range(0, 1000000);
	let id = format_radix(id, 36);
	id.substring(id.len() - 3, id.len()).into()
}
