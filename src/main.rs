use futures::future;
use state::AppState;
use std::time::Duration;
use tokio::{runtime, time};

mod error;
mod spide;
mod state;
mod web;

async fn create_timer_task(state: AppState) {
	let mut tid = time::interval(Duration::from_secs(100000));
	loop {
		tid.tick().await;
		if let Err(e) = spide::create_spide_task(state.clone()).await {
			eprintln!("{e}")
		}
	}
}

fn main() {
	let rt = runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();

	let state = state::create_init_state();

	rt.block_on(async move {
		let timer = create_timer_task(state.clone());
		let web = web::create_web_task(state);
		future::join(timer, web).await;
	});
}
