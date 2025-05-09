use crate::data::error::{AppError, Result};
use ntex::web::{self, Responder, types::Json};
use serde::Deserialize;

mod state;

#[derive(Debug, Deserialize)]
struct RegisterBody {
	nick: String,
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

	let session = state.register_user(&body.nick, &body.password).await?;
	Ok(Json(session))
}

#[web::post("/login")]
async fn login_api() -> impl Responder {
	"this is login"
}

pub fn api() -> web::Scope<web::error::DefaultError> {
	web::scope("/auth").service(register_api).service(login_api)
}
