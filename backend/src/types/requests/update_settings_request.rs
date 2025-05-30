use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSettingsRequest {
    pub user_id: u64,
    pub name: String,
    pub picture: Option<String>,
}
