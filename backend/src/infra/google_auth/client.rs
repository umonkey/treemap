use super::response::AuthResponse;
use crate::types::{Error, Result};
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v1/userinfo";

pub struct GoogleAuthClient {}

impl GoogleAuthClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn verify_token(&self, token: &str) -> Result<AuthResponse> {
        let client = Client::new();
        let headers = self.get_headers(token)?;

        let req = client.get(USERINFO_URL).headers(headers);

        debug!("Request: {req:?}");

        let res = match req.send().await {
            Ok(res) => res,

            Err(e) => {
                debug!("Failed to get user info from Google: {e:?}");
                return Err(Error::GoogleUserInfo);
            }
        };

        let info = res.json::<AuthResponse>().await.map_err(|e| {
            debug!("Failed to parse Google response: {e:?}");
            Error::GoogleUserInfo
        })?;

        Ok(info)
    }

    fn get_headers(&self, token: &str) -> Result<HeaderMap> {
        let auth_header = match HeaderValue::from_str(format!("Bearer {token}").as_str()) {
            Ok(h) => h,

            Err(e) => {
                debug!("Failed to create auth header: {e}");
                return Err(Error::BadAuthToken);
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", auth_header);
        Ok(headers)
    }
}
