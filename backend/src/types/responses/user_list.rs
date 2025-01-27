use crate::types::{UserRecord, UserResponse};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UserList {
    pub users: Vec<UserResponse>,
}

impl UserList {
    pub fn new() -> Self {
        Self { users: vec![] }
    }

    pub fn with_users(&self, users: &[UserRecord]) -> Self {
        let records = users.iter().map(UserResponse::from).collect();

        Self { users: records }
    }
}
