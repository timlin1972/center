use std::sync::{Arc, Mutex};

use crate::{reg, user};

pub struct AppState {
    pub reg_list: Arc<Mutex<reg::RegList>>,
    pub user_list: Arc<Mutex<user::UserList>>,
}
