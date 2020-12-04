use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::Builder::from_default_env()
	    .filter(None, log::LevelFilter::Info)
	    .init();
	Ok(gun::start().await?)
}
