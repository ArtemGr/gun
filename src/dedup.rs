#[cfg(feature = "std")]
use std::{
	sync::{Arc, Mutex},
	thread,
};
use core::{char, str, time::Duration};

use fnv::FnvHashMap;
use rand::prelude::*;

use crate::util::timestamp;

const MAX: f64 = 1.0;
const AGE: f64 = 9.0;

// #[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub struct Dedup {
	timeline: Arc<Mutex<FnvHashMap<String, f64>>>,
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
				thread::sleep(Duration::from_secs(1));

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
	let radix = 36;
	let mut n = rand::thread_rng().gen_range(radix*radix, radix*radix*radix);
	let mut bytes = [0; 3];

    for i in 0..3 {
        let m = n % radix;
        n = n / radix;

        bytes[i] = char::from_digit(m, radix).unwrap() as u8;
    }

    str::from_utf8(&bytes).unwrap().into()
}
