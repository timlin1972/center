use std::sync::{Arc, Mutex};

use crate::reg;

pub struct AppState {
    pub reg_list: Arc<Mutex<reg::RegList>>,
}
