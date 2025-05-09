use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileConfigDatabase {
	host: String,
	user: String,
	password: String,
	database: String,
}

#[derive(Debug, Deserialize)]
pub struct FileConfig {
	port: u16,
	database: FileConfigDatabase,
}

impl FileConfig {
	pub fn read_default_config() -> Result<Self, config::ConfigError> {
		const CONFIG_PATH: &str = "config/config.toml";

		let config =
			config::Config::builder().add_source(config::File::with_name("config/default.toml"));
		let config = if std::fs::exists(CONFIG_PATH).ok() == Some(true) {
			config.add_source(config::File::with_name(CONFIG_PATH))
		} else {
			config
		};

		config.build()?.try_deserialize()
	}
}
