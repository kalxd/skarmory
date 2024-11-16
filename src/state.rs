use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct JandanPic {
	link: String,
}

#[derive(Debug)]
pub struct State {
	pics: Vec<JandanPic>,
}

pub type AppState = Arc<RwLock<State>>;

pub fn create_init_state() -> AppState {
	Arc::new(RwLock::new(State { pics: vec![] }))
}
