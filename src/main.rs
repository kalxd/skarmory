mod base;

use base::Result;

#[ntex::main]
async fn main() -> Result<()> {
	let config = base::FileConfig::read_default_config()?;
	let pool = config.connect_to_db().await?;

	dbg!(config);
	dbg!(pool);
	Ok(())
}
