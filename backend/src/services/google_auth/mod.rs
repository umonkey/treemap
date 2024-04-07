use log::{debug, info};
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

use crate::Result;
use crate::errors::Error;
use crate::objects::{GoogleUserinfoResponse, LoginGoogleRequest, LoginResponse, UserInfo};
use crate::services::Database;
use crate::utils::get_unique_id;

pub struct GoogleAuth {
    db: Arc<dyn Database>,
}

impl GoogleAuth {
    pub async fn init(db: &Arc<dyn Database>) -> Self {
        Self {
            db: db.clone(),
        }
    }

    pub async fn login(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        debug!("Authenticating a Google user.");

        let userinfo = self.get_google_userinfo(req).await?;
        let user = self.get_user(&userinfo).await?;

        Ok(LoginResponse {
            token: user.id.to_string(),
            name: user.name,
            picture: user.picture,
        })
    }

    async fn get_user(&self, userinfo: &GoogleUserinfoResponse) -> Result<UserInfo> {
        if let Some(user) = self.db.find_user_by_email(&userinfo.email).await? {
            debug!("Found a user with email {}.", userinfo.email);
            return Ok(user);
        }

        debug!("Creating a new user with email {}.", userinfo.email);

        let user = UserInfo {
            id: get_unique_id()?,
            email: userinfo.email.clone(),
            name: userinfo.name.clone(),
            picture: userinfo.picture.clone(),
        };

        self.db.add_user(&user).await?;

        info!("Created a new user with email {} and id {}.", user.email, user.id);

        Ok(user)
    }

    async fn get_google_userinfo(&self, req: LoginGoogleRequest) -> Result<GoogleUserinfoResponse> {
        let client = reqwest::Client::new();

        let req = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .headers(self.get_login_headers(req)?);

        let res = match req.send().await {
            Ok(res) => res,

            Err(e) => {
                debug!("Failed to get use info from Google: {}", e);
                return Err(Error::GoogleUserInfo);
            }
        };

        match res.json::<GoogleUserinfoResponse>().await {
            Ok(u) => Ok(u),

            Err(e) => {
                debug!("Failed to parse Google response: {}", e);
                Err(Error::GoogleUserInfo)
            }
        }
    }

    fn get_login_headers(&self, req: LoginGoogleRequest) -> Result<HeaderMap> {
        let auth_header = match HeaderValue::from_str(format!("Bearer {}", req.token).as_str()) {
            Ok(h) => h,

            Err(e) => {
                debug!("Failed to create auth header: {}", e);
                return Err(Error::BadAuthToken);
            },
        };

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", auth_header);
        Ok(headers)
    }
}
