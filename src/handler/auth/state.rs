use crate::data::{
	AppEnv, User,
	db::{Gender, SaltPassword, Uuid},
	error::{AppError, Result},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SessionUser {
	token: Uuid,
	user: User,
}

#[derive(Debug, Clone, drv::State)]
pub struct AuthState(AppEnv);

impl AuthState {
	pub async fn register_user(
		&self,
		nickname: &str,
		password: &str,
		gender: &Gender,
	) -> Result<SessionUser> {
		let salt_password = SaltPassword::new(password, self.0.salt.as_deref());

		let result = sqlx::query!(
			r#"
insert into "user" (nick, password, gender)
values ($1, md5($2), $3)
on conflict do nothing
"#,
			nickname,
			salt_password as SaltPassword,
			gender as &Gender
		)
		.execute(&self.0.db)
		.await?;

		if result.rows_affected() == 0 {
			return Err(AppError::forbid("该昵称已经被注册，请重新申请新昵称。"));
		}

		let user = sqlx::query_as!(
			User,
			r#"
select id, nick, gender as "gender: Gender"
from "user"
where nick = $1"#,
			nickname
		)
		.fetch_one(&self.0.db)
		.await?;

		let token: Uuid = sqlx::query_scalar!(
			r#"
insert into session (user_id, token)
values ($1, gen_random_uuid())
returning token as "token!: Uuid"
"#,
			user.id
		)
		.fetch_one(&self.0.db)
		.await?;

		Ok(SessionUser { token, user })
	}

	pub async fn login(&self, nick: &str, password: &str) -> Result<SessionUser> {
		let salt_password = SaltPassword::new(password, self.0.salt.as_deref());
		let user = sqlx::query_as!(
			User,
			r#"
select id, nick, gender as "gender: Gender"
from "user"
where nick = $1 and password = md5($2)"#,
			nick,
			salt_password as SaltPassword
		)
		.fetch_optional(&self.0.db)
		.await?
		.ok_or(AppError::no_auth("用户不存在或密码不正确！"))?;

		let token = sqlx::query_scalar!(
			r#"
insert into "session" (user_id, token)
values ($1, gen_random_uuid())
returning token as "token!: Uuid"
"#,
			user.id
		)
		.fetch_one(&self.0.db)
		.await?;

		Ok(SessionUser { token, user })
	}
}
