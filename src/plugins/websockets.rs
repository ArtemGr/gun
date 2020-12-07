use anyhow::Result;

use crate::{GunBuilder, GunOptions, GunPlugin};

pub struct Websockets {}

impl Websockets {
	pub fn new() -> Self {
		Self {}
	}
}

impl GunPlugin for Websockets {
	fn start(&self, options: &GunOptions) -> Result<()> {
		println!("Websockets");
		Ok(())
	}
	fn emit(&self, _: String) { todo!() }
	fn wait_for_data(&self, timeout: f64) -> Result<String> { todo!() }
}

pub fn plug_into(gun: &mut GunBuilder) {
	gun.plugin = std::rc::Rc::new(Websockets::new());
}
