mod data;

use data::{config, error::Result};

#[ntex::main]
async fn main() -> Result<()> {
	let config = config::FileConfig::read_default_config()?;
	let pool = config.connect_to_db().await?;

	dbg!(config);
	dbg!(pool);
	Ok(())
}
