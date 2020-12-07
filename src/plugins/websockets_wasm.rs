use anyhow::Result;

use crate::{GunBuilder, GunOptions, GunPlugin};

pub struct WebsocketsWASM {}

impl WebsocketsWASM {
	pub fn new() -> Self {
		Self {}
	}
}

impl GunPlugin for WebsocketsWASM {
	fn start(&self, options: &GunOptions) -> Result<()> {
		println!("Websockets WASM");
		Ok(())
	}
	fn emit(&self, _: String) { todo!() }
	fn wait_for_data(&self, timeout: f64) -> Result<String> { todo!() }
}

pub fn plug_into(gun: &mut GunBuilder) {
	gun.plugin = std::rc::Rc::new(Websockets::new());
}
