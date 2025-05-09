use ntex::web;

pub mod auth;

pub fn api() -> web::Scope<web::error::DefaultError> {
	web::scope("/").service(auth::api())
}
