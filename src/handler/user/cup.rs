use ntex::web::{self, DefaultError, Scope, types::Json};
use serde::{Deserialize, Serialize};

use crate::data::{
	AppEnv, User,
	error::{AppError, Result},
};

#[derive(drv::State)]
struct State(AppEnv);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Cup {
	id: i32,
	user_id: i32,
	volum: i32,
	color: String,
}

#[derive(Debug, Deserialize)]
struct CreateCupBody {
	volum: u32,
	color: String,
}

#[web::post("/create")]
async fn create_cup_api(user: User, state: State, body: Json<CreateCupBody>) -> Result<Json<Cup>> {
	let volumn = i32::try_from(body.volum)
		.map_err(|_| AppError::forbid(&format!("无法正确处理杯子容量值：{}。", body.volum)))?;

	let cup = sqlx::query_as!(
		Cup,
		r#"
insert into cup (user_id, volum, color)
values ($1, $2, $3)
returning id, user_id, volum, color
"#,
		user.id,
		volumn,
		&body.color
	)
	.fetch_one(&state.0.db)
	.await?;

	Ok(Json(cup))
}

pub fn api() -> Scope<DefaultError> {
	web::scope("/cup").service(create_cup_api)
}
