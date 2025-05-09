use crate::data::AppEnv;
use ntex::web::{ErrorRenderer, FromRequest, error::StateExtractorError};

#[derive(Debug, Clone)]
pub struct AuthState(AppEnv);

impl<E: ErrorRenderer> FromRequest<E> for AuthState {
	type Error = StateExtractorError;

	async fn from_request(
		req: &ntex::web::HttpRequest,
		_: &mut ntex::http::Payload,
	) -> Result<Self, Self::Error> {
		match req.app_state::<AppEnv>() {
			Some(state) => Ok(Self(state.clone())),
			None => Err(StateExtractorError::NotConfigured),
		}
	}
}

impl AuthState {
	pub async fn register_user(nickname: &str, password: &str) {}
}
