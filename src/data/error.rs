#[derive(Debug)]
pub enum AppError {
	DBErr(String),
	BootErr(String),
}

impl std::fmt::Display for AppError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::DBErr(s) => write!(f, "DB Err: {s}"),
			Self::BootErr(s) => write!(f, "{s}"),
		}
	}
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
	fn from(value: std::io::Error) -> Self {
		Self::BootErr(value.to_string())
	}
}

impl From<config::ConfigError> for AppError {
	fn from(value: config::ConfigError) -> Self {
		Self::BootErr(value.to_string())
	}
}

impl From<sqlx::Error> for AppError {
	fn from(value: sqlx::Error) -> Self {
		Self::DBErr(value.to_string())
	}
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;
