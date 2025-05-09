use ntex::web::{self, Responder};

#[web::post("/register")]
async fn register_api() -> impl Responder {
	"this is register"
}

#[web::post("/login")]
async fn login_api() -> impl Responder {
	"this is login"
}

pub fn api() -> web::Scope<web::error::DefaultError> {
	web::scope("/auth").service(register_api).service(login_api)
}
