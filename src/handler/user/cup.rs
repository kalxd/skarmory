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
	nick: Option<String>,
	volum: i32,
	color: String,
}

#[derive(Debug, Deserialize)]
struct CreateCupBody {
	volum: u32,
	nick: Option<String>,
	color: String,
}

#[web::post("/create")]
async fn create_cup_api(user: User, state: State, body: Json<CreateCupBody>) -> Result<Json<Cup>> {
	let volumn = i32::try_from(body.volum).map_err(|_| AppError::forbid("请输入正确的杯子容量"))?;

	let cup = sqlx::query_as!(
		Cup,
		r#"
insert into cup (user_id, volum, color, nick)
values ($1, $2, $3, $4)
returning id, user_id, volum, color, nick
"#,
		user.id,
		volumn,
		&body.color,
		body.nick
	)
	.fetch_one(&state.0.db)
	.await?;

	Ok(Json(cup))
}

pub fn api() -> Scope<DefaultError> {
	web::scope("/cup").service(create_cup_api)
}
