use db::{Gender, Uuid};
use ntex::web::{ErrorRenderer, FromRequest};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

pub mod config;
pub mod db;
pub mod error;

use error::AppError;

#[derive(Debug, Clone)]
pub struct AppEnv {
	pub db: PgPool,
	pub salt: Arc<Option<String>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
	pub id: i32,
	pub nick: String,
	pub gender: Gender,
}

impl<E: ErrorRenderer> FromRequest<E> for User {
	type Error = AppError;

	async fn from_request(
		req: &ntex::web::HttpRequest,
		_: &mut ntex::http::Payload,
	) -> Result<Self, Self::Error> {
		let e = AppError::no_auth("尚未登录！");

		let token = req
			.headers()
			.get("X-Token")
			.ok_or(e.clone())?
			.to_str()
			.map_err(|e| AppError::no_auth(&e.to_string()))?;

		let app_env = req
			.app_state::<AppEnv>()
			.ok_or(AppError::internal("AppEnv注入不成功！"))?;

		let token = Uuid::try_from(token).map_err(|_| e.clone())?;

		let user = sqlx::query_as!(
			User,
			r#"
select u.id, u.nick, u.gender as "gender: Gender" from "user" as u
inner join session as s on s.user_id = u.id and s.token = $1
"#,
			token as Uuid
		)
		.fetch_optional(&app_env.db)
		.await?
		.ok_or(e)?;

		Ok(user)
	}
}
