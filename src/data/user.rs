use super::error::AppError;
use ntex::web::FromRequest;
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
	pub id: i32,
	pub nick: String,
}

impl FromRequest<AppError> for User {
	type Error = AppError;

	async fn from_request(
		req: &ntex::web::HttpRequest,
		payload: &mut ntex::http::Payload,
	) -> Result<Self, Self::Error> {
		let token = req
			.headers()
			.get("X-Token")
			.ok_or(AppError::no_auth("未设置token！"))?
			.to_str()
			.map_err(|e| AppError::no_auth(&e.to_string()))?;
		todo!()
	}
}
