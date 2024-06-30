use crate::types::database::UserRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub picture: String,
}

impl From<UserRecord> for UserResponse {
    fn from(record: UserRecord) -> Self {
        UserResponse {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
        }
    }
}

impl From<&UserRecord> for UserResponse {
    fn from(record: &UserRecord) -> Self {
        UserResponse {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
        }
    }
}
