use crate::data::{
	db::Gender,
	error::{AppError, Result},
};
use ntex::web::{self, DefaultError, types::Json};
use serde::Deserialize;

mod state;

#[derive(Debug, Deserialize)]
struct RegisterBody {
	nick: String,
	gender: Gender,
	password: String,
	repassword: String,
}

#[web::post("/register")]
async fn register_api(
	state: state::AuthState,
	body: Json<RegisterBody>,
) -> Result<Json<state::SessionUser>> {
	if body.password != body.repassword {
		return Err(AppError::forbid("两次密码不一致！"));
	}

	let session = state
		.register_user(&body.nick, &body.password, &body.gender)
		.await?;
	Ok(Json(session))
}

#[derive(Debug, Deserialize)]
struct LoginBody {
	nick: String,
	password: String,
}

#[web::post("/login")]
async fn login_api(
	state: state::AuthState,
	body: Json<LoginBody>,
) -> Result<Json<state::SessionUser>> {
	state.login(&body.nick, &body.password).await.map(Json)
}

pub fn api() -> web::Scope<DefaultError> {
	web::scope("/auth").service(register_api).service(login_api)
}
