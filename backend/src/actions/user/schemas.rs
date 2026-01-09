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
            name: record.name.clone(),
            picture: record.picture.clone(),
        }
    }
}

impl From<&User> for UserRead {
    fn from(record: &User) -> Self {
        UserRead {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
        }
    }
}
