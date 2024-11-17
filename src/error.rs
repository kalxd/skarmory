use axum::{
	http,
	response::{IntoResponse, Response},
};
use std::fmt::Display;

/// 最简单的错处收集。
#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "internal error: {}", self.0)
	}
}

impl<E: std::error::Error> From<E> for Error {
	fn from(value: E) -> Self {
		Self(value.to_string())
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		(http::StatusCode::BAD_GATEWAY, self.0).into_response()
	}
}

pub type Result<T> = std::result::Result<T, Error>;
