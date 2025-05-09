use sqlx::PgPool;

pub mod config;
pub mod db;
pub mod error;

#[derive(Debug, Clone)]
pub struct AppEnv {
	pub db: PgPool,
}
