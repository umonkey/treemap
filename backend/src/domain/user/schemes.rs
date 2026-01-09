use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub name: String,
    pub picture: String,
}
