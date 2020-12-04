use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::init();
	Ok(gun::start().await?)
}
