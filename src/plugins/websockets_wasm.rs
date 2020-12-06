use anyhow::Result;

use crate::{GunBuilder, GunFunctions};

pub fn start() -> Result<()> {
	println!("Websockets WASM");
	Ok(())
}

pub fn plug_into(gun: &mut GunBuilder) {
	gun.functions = GunFunctions {
		start,
	}
}
