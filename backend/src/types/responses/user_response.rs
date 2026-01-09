use crate::domain::user::User;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub picture: String,
}

impl From<User> for UserResponse {
    fn from(record: User) -> Self {
        UserResponse {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
        }
    }
}

impl From<&User> for UserResponse {
    fn from(record: &User) -> Self {
        UserResponse {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
        }
    }
}
