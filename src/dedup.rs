use std::sync::{Arc, Mutex};
use core::{char, str, time::Duration};

use async_std::task;
use fnv::FnvHashMap;
use rand::prelude::*;

use crate::util::{timestamp, Str};

const MAX: f64 = 1.0;
const AGE: f64 = 9.0;

pub struct Dedup {
	timeline: Arc<Mutex<FnvHashMap<Str, f64>>>,
	timeout: Arc<Mutex<bool>>,
}

impl Dedup {
	pub fn new() -> Self {
		Self {
			timeline: Arc::new(Mutex::new(FnvHashMap::default())),
			timeout: Arc::new(Mutex::new(false)),
		}
	}

	pub fn check(&mut self, soul: &str) -> Option<Str> {
		if self.timeline.lock().unwrap().contains_key(soul.into()) {
			Some(self.track(soul))
		} else {
			None
		}
	}

	pub fn track(&mut self, soul: &str) -> Str {
		self.timeline.lock().unwrap().insert(soul.into(), timestamp());

		if !*self.timeout.lock().unwrap() {
			*self.timeout.lock().unwrap() = true;

			let timeline = self.timeline.clone();
			let timeout = self.timeout.clone();

			task::spawn(async move {
				task::sleep(Duration::from_secs(1)).await;

				for (soul, time) in &*timeline.lock().unwrap() {
					if AGE > timestamp() - time {
						continue;
					}

					timeline.lock().unwrap().remove(soul);
				}

				*timeout.lock().unwrap() = false;
			});
		}

		Str::from(soul)
	}
}

pub fn random_soul() -> Str {
	let radix = 36;
	let mut n = rand::thread_rng().gen_range(radix*radix, radix*radix*radix);
	let mut bytes = [0; 3];

    for i in 0..3 {
        let m = n % radix;
        n = n / radix;

        bytes[i] = char::from_digit(m, radix).unwrap() as u8;
    }

    Str::from_str(str::from_utf8(&bytes).unwrap())
}
