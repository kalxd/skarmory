mod base;

#[ntex::main]
async fn main() -> Result<(), config::ConfigError> {
	let config = base::FileConfig::read_default_config()?;

	dbg!(config);
	Ok(())
}
