use anyhow::Result;
use gun::{plugins, GunBuilder};

fn main() -> Result<()> {
	env_logger::Builder::from_default_env()
	    .filter(None, log::LevelFilter::Info)
	    .init();

	let mut gun = GunBuilder::new();
	plugins::tungstenite::plug_into(&mut gun);
	let gun = gun.build();

	gun.start()
}
