use config::{Config, File};
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgConnectOptions};
use std::fs;

use super::error::Result;

#[derive(Debug, Deserialize)]
pub struct FileConfigDatabase {
	host: String,
	port: u16,
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

		let config = Config::builder().add_source(File::with_name("config/default.toml"));
		let config = if fs::exists(CONFIG_PATH).ok() == Some(true) {
			config.add_source(File::with_name(CONFIG_PATH))
		} else {
			config
		};

		config.build()?.try_deserialize().map_err(Into::into)
	}

	pub async fn connect_to_db(&self) -> Result<PgPool> {
		let option = PgConnectOptions::new()
			.host(&self.database.host)
			.port(self.database.port)
			.username(&self.database.user)
			.password(&self.database.password)
			.database(&self.database.database);

		PgPool::connect_with(option).await.map_err(Into::into)
	}
}
