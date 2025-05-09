use crate::data::db::Uuid;
use ntex::web::{self, Responder, types::Json};

mod state;

#[web::post("/register")]
async fn register_api(state: state::AuthState) -> impl Responder {
	"hello world"
}

#[web::post("/login")]
async fn login_api() -> impl Responder {
	"this is login"
}

pub fn api() -> web::Scope<web::error::DefaultError> {
	web::scope("/auth").service(register_api).service(login_api)
}
