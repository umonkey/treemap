use crate::domain::user::User;
use crate::types::UserResponse;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UserList {
    pub users: Vec<UserResponse>,
}

impl From<Vec<User>> for UserList {
    fn from(users: Vec<User>) -> Self {
        let records = users.iter().map(UserResponse::from).collect();
        Self { users: records }
    }
}
