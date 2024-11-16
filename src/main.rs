use std::{thread, time::Duration};
use tokio::{runtime, time};

mod state;

async fn task_timer(i: usize) {
	let mut tid = time::interval(Duration::from_secs(10));
	loop {
		tid.tick().await;

		println!("doing {i}");
	}
}

fn create_timer_task() {
	let rt = runtime::Builder::new_current_thread()
		.enable_time()
		.build()
		.unwrap();

	rt.block_on(task_timer(1));
}

fn create_web_task() {
	let rt = runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();
	rt.block_on(task_timer(2));
}

fn main() {
	let timer = thread::spawn(create_timer_task);
	let web = thread::spawn(create_web_task);

	timer.join().unwrap();
	web.join().unwrap();
}
