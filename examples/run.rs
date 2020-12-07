use anyhow::Result;
use gun::GunBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ASDF {
	name: String,
}

fn main() -> Result<()> {
	env_logger::Builder::from_default_env()
	    .filter(None, log::LevelFilter::Info)
	    .init();

	let gun = GunBuilder::new();
	let gun = gun.build();

	gun.start()?;

	std::thread::sleep(std::time::Duration::from_secs(5));

	loop {
		if let Ok(asdf) = gun.get("ASDF").value::<ASDF>() {
			log::info!("{:?}", asdf);
			break;
		}
	}

	gun.block();

	Ok(())
}
