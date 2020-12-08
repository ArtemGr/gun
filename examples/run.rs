use std::{sync::Arc, thread, time::Duration};

use anyhow::Result;
use gun::GunBuilder;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
	name: String,
	color: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::Builder::from_default_env()
	    .filter(None, log::LevelFilter::Info)
	    .init();

	let gun = GunBuilder::new().peers(&["ws://e2eec.herokuapp.com/gun"]);
	let gun = gun.build();
	let gun = Arc::new(gun);

	let gun_clone = gun.clone();
	tokio::spawn(async move {
		match gun_clone.start().await {
			Ok(_) => (),
			Err(err) => {
				log::error!("{}", err);
				std::process::exit(1);
			}
		}
	});

	gun.get("cat").put(Cat { name: "henry".into(), color: "grey".into() }).await;

	gun.get("cat").once(|cat: Cat| {
		log::info!("{:?}", cat);
	}).await;

	Ok(())
}
