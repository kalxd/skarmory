use sqlx::PgPool;

pub mod config;
pub mod db;
pub mod error;

#[derive(Clone)]
pub struct AppEnv {
	pub db: PgPool,
}
