use crate::domain::user::User;
use crate::types::UserRead;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UserList {
    pub users: Vec<UserRead>,
}

impl From<Vec<User>> for UserList {
    fn from(users: Vec<User>) -> Self {
        let records = users.iter().map(UserRead::from).collect();
        Self { users: records }
    }
}
