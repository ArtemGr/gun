use anyhow::Result;

use crate::{GunBuilder, GunFunctions};

pub fn start(peers: &[&str]) -> Result<()> {
	println!("Websockets");
	Ok(())
}

pub fn plug_into(gun: &mut GunBuilder) {
	gun.functions = GunFunctions {
		start,
	}
}
