use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Message {
    Identify {
        username: String,
    },
    Response {
        operation: Operation,
        result: ResponseResult,
        extra: String,
    },
    NewUser {
        username: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    Identify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseResult {
    Success,
    UserAlreadyExists,
}

impl Message {
    pub fn identify(username: impl Into<String>) -> Self {
        Self::Identify {
            username: username.into(),
        }
    }

    pub fn identify_success(username: impl Into<String>) -> Self {
        Self::Response {
            operation: Operation::Identify,
            result: ResponseResult::Success,
            extra: username.into(),
        }
    }

    pub fn identify_user_already_exists(username: impl Into<String>) -> Self {
        Self::Response {
            operation: Operation::Identify,
            result: ResponseResult::UserAlreadyExists,
            extra: username.into(),
        }
    }

    pub fn new_user(username: impl Into<String>) -> Self {
        Self::NewUser {
            username: username.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_json_shape() {
        let msg = Message::identify("Kimberly");
        let json = serde_json::to_string(&msg).expect("serialize identify");

        assert_eq!(json, r#"{"type":"IDENTIFY","username":"Kimberly"}"#);
    }

    #[test]
    fn identify_success_response_json_shape() {
        let msg = Message::identify_success("Kimberly");
        let json = serde_json::to_string(&msg).expect("serialize response");

        assert_eq!(
            json,
            r#"{"type":"RESPONSE","operation":"IDENTIFY","result":"SUCCESS","extra":"Kimberly"}"#
        );
    }

    #[test]
    fn new_user_json_shape() {
        let msg = Message::new_user("Kimberly");
        let json = serde_json::to_string(&msg).expect("serialize new user");

        assert_eq!(json, r#"{"type":"NEW_USER","username":"Kimberly"}"#);
    }
}
