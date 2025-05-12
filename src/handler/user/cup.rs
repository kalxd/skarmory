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
	value: i64,
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
returning id, user_id, volum, color, nick, 0 as "value!"
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
	.await? as i32;

	let vol = match body.op {
		CupOp::PourIn if body.value + cur_volum > 100 => {
			Err(AppError::forbid("无法加水：超出杯子容量！"))
		}
		CupOp::PourIn => Ok(body.value),
		CupOp::PourOut if body.value > cur_volum => {
			Err(AppError::forbid("无法倒出过的水：超出剩余水量！"))
		}
		CupOp::PourOut => Ok(-body.value),
		CupOp::Drink if body.value > cur_volum => {
			Err(AppError::forbid("无法喝过多的水：超出剩余水量！"))
		}
		CupOp::Drink => Ok(-body.value),
	}?;

	sqlx::query!(
		r#"
insert into cup_operator (cup_id, value, op)
values ($1, $2, $3)
"#,
		cup.id,
		vol,
		&body.op as &CupOp
	)
	.execute(&mut *tx)
	.await?;

	tx.commit().await?;

	Ok(Json(()))
}

#[web::get("/list")]
async fn list_cup(user: User, state: State) -> Result<Json<Vec<CupWithVol>>> {
	let xs = sqlx::query_as!(
		CupWithVol,
		r#"
select cup.id, cup.user_id, cup.nick, cup.volum, cup.color, op.value as "value!"
from
cup,
lateral (select coalesce(sum(value), 0) as value from cup_operator where cup_id = cup.id) as op
where cup.user_id = $1
"#,
		user.id
	)
	.fetch_all(&state.0.db)
	.await?;

	Ok(Json(xs))
}

pub fn api() -> Scope<DefaultError> {
	web::scope("/cup")
		.service(create_cup_api)
		.service(create_cup_op_api)
		.service(list_cup)
}
