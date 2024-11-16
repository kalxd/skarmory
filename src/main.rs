use futures::future;
use std::time::Duration;
use tokio::{runtime, time};

mod state;
mod web;

async fn create_timer_task(i: usize) {
	let mut tid = time::interval(Duration::from_secs(10));
	loop {
		tid.tick().await;

		println!("doing {i}");
	}
}

fn main() {
	let rt = runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();

	let state = state::create_init_state();

	rt.block_on(async move {
		let timer = create_timer_task(1);
		let web = web::create_web_task(state);
		future::join(timer, web).await;
	});
}
