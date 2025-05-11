use ntex::web::{self, DefaultError, Scope, types::Json};

use crate::data::User;

#[web::get("/info")]
async fn info_api(user: Option<User>) -> Json<Option<User>> {
	Json(user)
}

pub fn api() -> Scope<DefaultError> {
	web::scope("/user").service(info_api)
}
