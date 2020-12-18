use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value as JSON;

use crate::GunOptions;

#[async_trait]
pub trait GunPlugin {
	async fn start<'a>(&self, options: &GunOptions<'a>) -> Result<()>;
	fn emit(&self, data: String);
	fn check(&self, key: &str) -> Option<JSON>;
	fn wait_for_connection(&self);
	fn graph(&self) -> JSON;
}

pub struct DummyPlugin {}

impl DummyPlugin {
	pub fn new() -> Self { Self {} }
}

#[async_trait]
impl GunPlugin for DummyPlugin {
	async fn start<'a>(&self, _options: &GunOptions<'a>) -> Result<()> { unimplemented!() }
	fn emit(&self, _data: String) { unimplemented!() }
	fn check(&self, _key: &str) -> Option<JSON> { unimplemented!() }
	fn wait_for_connection(&self) { unimplemented!() }
	fn graph(&self) -> JSON { unimplemented!() }
}
