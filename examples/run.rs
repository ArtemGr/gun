use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	Ok(gun::start().await?)
}
