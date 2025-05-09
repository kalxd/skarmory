use crate::data::{
	AppEnv,
	db::{SaltPassword, Uuid},
	error::{AppError, Result},
	user::User,
};
use ntex::web::{ErrorRenderer, FromRequest, error::StateExtractorError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SessionUser {
	token: Uuid,
	user: User,
}

#[derive(Debug, Clone)]
pub struct AuthState(AppEnv);

impl<E: ErrorRenderer> FromRequest<E> for AuthState {
	type Error = StateExtractorError;

	async fn from_request(
		req: &ntex::web::HttpRequest,
		_: &mut ntex::http::Payload,
	) -> Result<Self, Self::Error> {
		match req.app_state::<AppEnv>() {
			Some(state) => Ok(Self(state.clone())),
			None => Err(StateExtractorError::NotConfigured),
		}
	}
}

impl AuthState {
	pub async fn register_user(&self, nickname: &str, password: &str) -> Result<SessionUser> {
		let salt_password = SaltPassword::new(password, self.0.salt.as_deref());

		let result = sqlx::query!(
			r#"
insert into "user" (nick, password)
values ($1, md5($2))
on conflict do nothing
"#,
			nickname,
			salt_password as SaltPassword
		)
		.execute(&self.0.db)
		.await?;

		if result.rows_affected() == 0 {
			return Err(AppError::forbid("该昵称已经被注册，请重新申请新昵称。"));
		}

		let user = sqlx::query_as!(
			User,
			r#"select id, nick from "user" where nick = $1"#,
			nickname
		)
		.fetch_one(&self.0.db)
		.await?;

		let token: Uuid = sqlx::query_scalar!(
			r#"insert into session (user_id, token) values ($1, gen_random_uuid()) returning token as "token!: Uuid" "#,
			user.id
		)
		.fetch_one(&self.0.db)
		.await?;

		Ok(SessionUser { token, user })
	}
}
