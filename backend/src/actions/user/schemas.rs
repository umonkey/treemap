use crate::domain::user::User;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UserList {
    pub users: Vec<UserRead>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserRead {
    pub id: String,
    pub name: String,
    pub picture: String,
    pub trees_count: i64,
    pub comments_count: i64,
    pub updates_count: i64,
    pub files_count: i64,
}

impl From<Vec<User>> for UserList {
    fn from(users: Vec<User>) -> Self {
        let records = users.iter().map(UserRead::from).collect();
        Self { users: records }
    }
}

impl From<User> for UserRead {
    fn from(record: User) -> Self {
        UserRead {
            id: record.id.to_string(),
            ..record.into()
        }
    }
}

impl From<&User> for UserRead {
    fn from(record: &User) -> Self {
        UserRead {
            id: record.id.to_string(),
            ..record.into()
        }
    }
}
