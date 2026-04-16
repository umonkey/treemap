use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleAuthCallbackPayload {
    pub access_token: String,
    pub state: String,
}
