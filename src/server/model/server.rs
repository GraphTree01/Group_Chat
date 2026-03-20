use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::shared::protocol::{Message, UserStatus};
use crate::shared::user::User;

pub enum IdentifyResult {
    Success { response: Message, new_user: Message },
    UserAlreadyExists { response: Message },
}

pub struct Server {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn identify(&self, username: String) -> std::io::Result<IdentifyResult> {
        let mut guard = self
            .users
            .lock()
            .map_err(|_| std::io::Error::other("user registry lock poisoned"))?;

        if guard.contains_key(&username) {
            return Ok(IdentifyResult::UserAlreadyExists {
                response: Message::identify_user_already_exists(username),
            });
        }

        guard.insert(
            username.clone(),
            User::new(username.clone(), UserStatus::Active),
        );

        Ok(IdentifyResult::Success {
            response: Message::identify_success(username.clone()),
            new_user: Message::new_user(username),
        })
    }

    pub fn disconnect(&self, username: &str) -> std::io::Result<()> {
        let mut guard = self
            .users
            .lock()
            .map_err(|_| std::io::Error::other("user registry lock poisoned"))?;
        guard.remove(username);
        Ok(())
    }

    pub fn add_user(&self, user: User) {
        if let Ok(mut guard) = self.users.lock() {
            guard.insert(user.username.clone(), user);
        }
    }
}
