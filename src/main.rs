mod base;

use base::Result;

#[ntex::main]
async fn main() -> Result<()> {
	let config = base::FileConfig::read_default_config()?;

	dbg!(config);
	Ok(())
}
