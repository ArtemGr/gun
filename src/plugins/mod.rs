#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub mod tungstenite;
#[cfg(not(target_arch = "wasm32"))]
pub mod websockets;
#[cfg(target_arch = "wasm32")]
pub mod websockets_wasm;

use anyhow::Result;

use crate::GunOptions;

pub trait GunPlugin {
	fn start(&self, options: &GunOptions) -> Result<()>;
	fn emit(&self, data: String);
	fn wait_for_data(&self, timeout: f64) -> Result<String>;
}
