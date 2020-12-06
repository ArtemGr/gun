use std::{
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};

use fnv::FnvHashMap;
use rand::prelude::*;
use substring::Substring;

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

	pub fn check(&mut self, soul: String) -> Option<String> {
		if self.timeline.lock().unwrap().contains_key(&soul) {
			Some(self.track(soul))
		} else {
			None
		}
	}

	pub fn track(&mut self, soul: String) -> String {
		self.timeline.lock().unwrap().insert(soul.clone(), timestamp());

		if !*self.timeout.lock().unwrap() {
			*self.timeout.lock().unwrap() = true;

			let timeline = self.timeline.clone();
			let timeout = self.timeout.clone();

			thread::spawn(move || {
				thread::sleep(Duration::from_millis(1000));

				for (soul, time) in &*timeline.lock().unwrap() {
					if AGE > timestamp() - time {
						continue;
					}

					timeline.lock().unwrap().remove(soul);
				}

				*timeout.lock().unwrap() = false;
			});
		}

		soul
	}
}

pub fn random_soul() -> String {
	let soul = rand::thread_rng().gen_range(0, 1000000);
	let soul = format_radix(soul, 36);
	soul.substring(soul.len() - 3, soul.len()).into()
}
