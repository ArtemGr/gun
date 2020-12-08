use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value as JSON;

use gun::{GunBuilder, GunOptions, plugins::GunPlugin};

pub struct WebsocketsWASM {}

impl WebsocketsWASM {
	pub fn new() -> Self {
		Self {}
	}

	pub fn plug_into(gun: &mut GunBuilder) {
		gun.plugin = Arc::new(Box::new(WebsocketsWASM::new()));
	}
}

#[async_trait]
impl GunPlugin for WebsocketsWASM {
	async fn start<'a>(&self, _options: &GunOptions<'a>) -> Result<()> {
		println!("Websockets WASM");
		Ok(())
	}
	fn emit(&self, _data: String) { todo!() }
	fn check(&self, _key: &str) -> Option<JSON> { todo!() }
	fn wait_for_connection(&self) { todo!() }
}
