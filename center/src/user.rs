use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

pub struct User {
    id: i32,
    user_info: UserInfo,
}

pub struct UserList {
    pub list: Vec<User>,
}

impl Default for UserList {
    fn default() -> Self {
        Self::new()
    }
}

impl UserList {
    pub fn new() -> UserList {
        let user = User {
            id: 0,
            user_info: UserInfo {
                username: "timlin".to_owned(),
                password: hash("timlin", DEFAULT_COST).unwrap(),
            },
        };
        UserList { list: vec![user] }
    }

    pub fn validator(&self, user_validate: &UserInfo) -> Option<i32> {
        for user in &self.list {
            if user.user_info.username == user_validate.username {
                if verify(&user_validate.password, &user.user_info.password).unwrap() {
                    return Some(user.id);
                } else {
                    return None;
                }
            }
        }

        None
    }
}
