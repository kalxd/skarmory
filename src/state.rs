use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct JandanPic {
	pub link: String,
}

#[derive(Debug)]
pub struct State {
	pub pics: Vec<JandanPic>,
}

pub type AppState = Arc<RwLock<State>>;

pub fn create_init_state() -> AppState {
	Arc::new(RwLock::new(State { pics: vec![] }))
}

pub fn write_state(state: &AppState, pic_list: Vec<String>) {
	let mut w = state.write().unwrap();
	w.pics = pic_list
		.into_iter()
		.map(|x| JandanPic { link: x })
		.collect();
}
