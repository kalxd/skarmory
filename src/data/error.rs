use ntex::{
	http::{Response, StatusCode},
	web::{self, WebResponseError},
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum AppError {
	Forbid(String),
	NoAuth(String),
	NotFound(String),
	Internal(String),
}

impl AppError {
	pub fn forbid(msg: &str) -> Self {
		Self::Forbid(msg.to_string())
	}

	pub fn no_auth(msg: &str) -> Self {
		Self::NoAuth(msg.to_string())
	}

	pub fn not_found(msg: &str) -> Self {
		Self::NotFound(msg.to_string())
	}

	pub fn internal(msg: &str) -> Self {
		Self::Internal(msg.to_string())
	}
}

impl std::fmt::Display for AppError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Forbid(s) => write!(f, "{s}"),
			Self::NoAuth(s) => write!(f, "{s}"),
			Self::NotFound(s) => write!(f, "{s}"),
			Self::Internal(s) => write!(f, "{s}"),
		}
	}
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
	fn from(value: std::io::Error) -> Self {
		Self::Internal(value.to_string())
	}
}

impl From<config::ConfigError> for AppError {
	fn from(value: config::ConfigError) -> Self {
		Self::Internal(value.to_string())
	}
}

impl From<sqlx::Error> for AppError {
	fn from(value: sqlx::Error) -> Self {
		Self::Internal(value.to_string())
	}
}

impl WebResponseError for AppError {
	fn status_code(&self) -> StatusCode {
		match self {
			Self::Forbid(_) => StatusCode::FORBIDDEN,
			Self::NoAuth(_) => StatusCode::UNAUTHORIZED,
			Self::NotFound(_) => StatusCode::NOT_FOUND,
			Self::Internal(_) => StatusCode::BAD_REQUEST,
		}
	}

	fn error_response(&self, _: &web::HttpRequest) -> Response {
		#[derive(Serialize)]
		struct Body {
			msg: String,
		}

		let status_code = self.status_code();
		let body = Body {
			msg: self.to_string(),
		};
		Response::build(status_code).json(&body)
	}
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;
