use ntex::web::{
	self, DefaultError, Scope,
	types::{Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::data::{
	AppEnv, User,
	db::CupOp,
	error::{AppError, Result},
};

#[derive(drv::State)]
struct State(AppEnv);

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct Cup {
	id: i32,
	user_id: i32,
	nick: Option<String>,
	volum: i32,
	color: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct CupWithVol {
	id: i32,
	user_id: i32,
	nick: Option<String>,
	volum: i32,
	color: String,
	usage: i32,
}

#[derive(Debug, Deserialize)]
struct CreateCupBody {
	volum: u32,
	nick: Option<String>,
	color: String,
}

#[web::post("/create")]
async fn create_cup_api(
	user: User,
	state: State,
	body: Json<CreateCupBody>,
) -> Result<Json<CupWithVol>> {
	let volumn = i32::try_from(body.volum).map_err(|_| AppError::forbid("请输入正确的杯子容量"))?;

	let cup = sqlx::query_as!(
		CupWithVol,
		r#"
insert into cup (user_id, volum, color, nick)
values ($1, $2, $3, $4)
returning id, user_id, volum, color, nick, 0 as "usage!"
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

#[derive(Debug, Deserialize)]
struct CreateCupOpBody {
	value: i32,
	op: CupOp,
}

#[web::post("/{cup_id}/op/create")]
async fn create_cup_op_api(
	user: User,
	state: State,
	cup_id: Path<i32>,
	body: Json<CreateCupOpBody>,
) -> Result<Json<()>> {
	let mut tx = state.0.db.begin().await?;

	let cup = sqlx::query_as!(
		Cup,
		r#"
select cup.id, cup.user_id, cup.nick, cup.volum, cup.color
from cup
where cup.id = $1 and cup.user_id = $2
for update
"#,
		*cup_id,
		&user.id
	)
	.fetch_optional(&mut *tx)
	.await?
	.ok_or(AppError::not_found("杯子不存在！"))?;

	let cur_volum = sqlx::query_scalar!(
		r#"
select coalesce(sum(value), 0) as "v!" from cup_operator
where cup_id = $1
"#,
		cup.id
	)
	.fetch_one(&mut *tx)
	.await?;

	dbg!(cur_volum);

	sqlx::query!(
		r#"
insert into cup_operator (cup_id, value, op)
values ($1, $2, $3)
"#,
		cup.id,
		&body.value,
		&body.op as &CupOp
	)
	.execute(&mut *tx)
	.await?;

	tx.commit().await?;

	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	web::scope("/cup")
		.service(create_cup_api)
		.service(create_cup_op_api)
}
