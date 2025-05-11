use ntex::web::{self, DefaultError};

pub mod auth;
pub mod user;

pub fn api() -> web::Scope<DefaultError> {
	web::scope("/").service(auth::api()).service(user::api())
}
