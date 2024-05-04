use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MeResponse {
    pub name: String,
    pub picture: String,
}
