use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserRecord {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub picture: String,
}
