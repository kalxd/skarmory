use sqlx::PgPool;
use std::sync::Arc;

pub mod config;
pub mod db;
pub mod error;
pub mod user;

#[derive(Debug, Clone)]
pub struct AppEnv {
	pub db: PgPool,
	pub salt: Arc<Option<String>>,
}
