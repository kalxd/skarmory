use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
	pub id: i32,
	pub nick: String,
}
