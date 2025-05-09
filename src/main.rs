use data::{config, error::Result};
use ntex::web;
use std::sync::Arc;

mod data;
mod handler;

#[ntex::main]
async fn main() -> Result<()> {
	let config = config::FileConfig::read_default_config()?;
	let pool = config.connect_to_db().await?;
	let state = data::AppEnv {
		db: pool,
		salt: Arc::new(config.salt),
	};

	web::HttpServer::new(move || web::App::new().state(state.clone()).service(handler::api()))
		.bind(("0.0.0.0", config.port))?
		.run()
		.await?;
	Ok(())
}
