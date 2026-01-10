use crate::domain::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub trees_count: i64,
    pub comments_count: i64,
    pub updates_count: i64,
    pub files_count: i64,
}

impl From<User> for MeResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            picture: user.email.clone(),
            trees_count: user.trees_count,
            comments_count: user.comments_count,
            updates_count: user.updates_count,
            files_count: user.files_count,
        }
    }
}
