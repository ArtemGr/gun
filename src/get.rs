use anyhow::{anyhow, Result};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

use crate::{
	dedup::random_soul,
	util::{timestamp, next_state, Plugin, METADATA, SOUL, STATE},
};

pub struct GunGet<'a> {
	#[cfg(feature = "std")]
	plugin: Plugin<'a>,
	key: &'a str,
}

impl<'a> GunGet<'a> {
	pub fn new(plugin: Plugin<'a>, key: &'a str) -> Self {
		Self {
			plugin,
			key,
		}
	}

	pub async fn once<T>(&self, cb: fn(T)) -> Result<()> where T: DeserializeOwned {
		let data = json!({
            SOUL: random_soul(),
            "get": { SOUL: self.key },
        }).to_string();
        
		self.plugin.wait_for_connection();

		loop {
            self.plugin.emit(data.clone());
            let begin = timestamp();
            loop {
                if let Some(data) = self.plugin.check(self.key) {
	                match serde_json::from_value(data) {
						Ok(res) => cb(res),
						Err(err) => return Err(anyhow!(err)),
					}

					return Ok(())
				}
                if timestamp() - begin > 10.0 {
                    break;
                }
            }
        }
	}

	pub async fn on<T>(&self, cb: fn(T)) where T: DeserializeOwned {
		let data = json!({
            SOUL: random_soul(),
            "get": { SOUL: self.key },
        }).to_string();

		self.plugin.wait_for_connection();
        self.plugin.emit(data.clone());

		loop {
            if let Some(data) = self.plugin.check(self.key) {
                match serde_json::from_value(data) {
					Ok(res) => cb(res),
					Err(_) => (),
				}
			}
		}
	}

	pub async fn put<T>(&self, data: T) -> Result<()> where T: Serialize {
		let mut data = json!(data);
		data[METADATA] = json!({
			SOUL: self.key,
			STATE: next_state(self.key, &data, self.plugin.graph()),
		});

		log::info!("{}", data[METADATA][STATE]);

		let data = json!({
			SOUL: random_soul(),
			"put": {
				self.key: data,
			},
		}).to_string();

		self.plugin.wait_for_connection();
		self.plugin.emit(data);

		Ok(())
	}
}
