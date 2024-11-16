use crate::state::AppState;
use axum::{self, extract::State, routing::get, Json, Router};

async fn first_pic(State(state): State<AppState>) -> Json<Vec<String>> {
	let xs = {
		let st = state.read().unwrap();
		st.pics.iter().map(|x| x.link.clone()).collect()
	};

	Json(xs)
}

fn jandan_routes() -> Router<AppState> {
	Router::new().route("/pics", get(first_pic))
}

pub async fn create_web_task(state: AppState) {
	let app = Router::new()
		.route("/", get(|| async { "hello world" }))
		.nest("/jandan", jandan_routes())
		.with_state(state);

	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
		.await
		.unwrap();

	axum::serve(listener, app).await.unwrap();
}
