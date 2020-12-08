use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value as JSON;

use crate::{GunBuilder, GunOptions, Plugin, plugins::GunPlugin};

pub struct WebsocketsWASM {}

impl WebsocketsWASM {
	pub fn new() -> Self {
		Self {}
	}
}

#[async_trait]
impl GunPlugin for WebsocketsWASM {
	async fn start<'a>(&self, options: &GunOptions<'a>) -> Result<()> {
		println!("Websockets WASM");
		Ok(())
	}
	async fn once(&self, _: &str) -> JSON { todo!() }
}

pub fn plug_into(gun: &mut GunBuilder) {
	gun.plugin = Plugin::new(Box::new(Websockets::new()));
}
