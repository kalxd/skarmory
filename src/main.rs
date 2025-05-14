use data::{config, error::Result};
use ntex::web;
use std::sync::Arc;

mod data;
mod handler;

fn init_logger() {
	use env_logger::Env;

	let env = Env::new();
	let env = env.filter_or("RUST_LOG", "debug");

	env_logger::init_from_env(dbg!(env));
}

#[ntex::main]
async fn main() -> Result<()> {
	init_logger();

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
