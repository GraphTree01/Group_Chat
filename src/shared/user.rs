use crate::shared::protocol::UserStatus;

pub struct User {
    pub username: String,
    pub status: UserStatus,
}

impl User {
    pub fn new(username: String, status: UserStatus) -> Self {
        Self { username, status }
    }
}
