#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub mod websockets_tokio;
// #[cfg(not(target_arch = "wasm32"))]
// pub mod websockets;
// #[cfg(target_arch = "wasm32")]
// pub mod websockets_wasm;

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
}
