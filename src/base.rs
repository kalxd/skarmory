use serde::Deserialize;

#[derive(Debug)]
pub enum AppError {
	BootErr(String),
}

impl std::fmt::Display for AppError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BootErr(s) => write!(f, "{s}"),
		}
	}
}

impl std::error::Error for AppError {}

impl From<config::ConfigError> for AppError {
	fn from(value: config::ConfigError) -> Self {
		Self::BootErr(value.to_string())
	}
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;

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
	pub fn read_default_config() -> Result<Self> {
		const CONFIG_PATH: &str = "config/config.toml";

		let config =
			config::Config::builder().add_source(config::File::with_name("config/default.toml"));
		let config = if std::fs::exists(CONFIG_PATH).ok() == Some(true) {
			config.add_source(config::File::with_name(CONFIG_PATH))
		} else {
			config
		};

		config.build()?.try_deserialize().map_err(Into::into)
	}
}
