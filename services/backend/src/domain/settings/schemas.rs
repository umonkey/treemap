use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSettingsRequest {
    pub user_id: u64,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Settings {
    pub name: String,
    pub description: Option<String>,
    pub bbox: Option<[f64; 4]>,
}
