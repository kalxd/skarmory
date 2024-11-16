use crate::state::AppState;
use axum::{self, routing::get, Router};

pub async fn create_web_task(state: AppState) {
	let app = Router::new().route("/", get(|| async { "hello world" }));
	let l = tokio::net::TcpListener::bind("127.0.0.1:3000")
		.await
		.unwrap();

	axum::serve(l, app).await.unwrap();
}
