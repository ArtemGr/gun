#[cfg(feature = "std")]
use std::rc::Rc;

use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::{
	dedup::random_soul,
	GunPlugin,
	util::{parse_json, METADATA, SOUL},
};

const TIMEOUT: f64 = 5.0; // Can let user define this later

pub struct GunGet<'a> {
	#[cfg(feature = "std")]
	plugin: Rc<dyn GunPlugin + 'a>,
	key: &'a str,
}

impl<'a> GunGet<'a> {
	pub fn new(plugin: Rc<dyn GunPlugin + 'a>, key: &'a str) -> Self {
		Self {
			plugin,
			key,
		}
	}

	pub fn value<T>(&self) -> Result<T> where T: DeserializeOwned {
		let data = json!({
			SOUL: random_soul(),
			"get": { SOUL: self.key },
		}).to_string();
		self.plugin.emit(data);

		let data = self.plugin.wait_for_data(TIMEOUT)?;
		let data = parse_json(&data).unwrap();
		let mut data = data["put"][self.key].clone();
		data[METADATA].take();

		match serde_json::from_value(data) {
			Ok(res) => Ok(res),
			Err(err) => Err(anyhow!(err)),
		}
	}
}
