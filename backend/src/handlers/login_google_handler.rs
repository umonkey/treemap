use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

const TOKEN_TTL: u64 = 30 * 86400; // 30 days

pub struct LoginGoogleHandler {
    users: Arc<UserRepository>,
    tokens: Arc<TokenService>,
}

impl LoginGoogleHandler {
    pub async fn handle(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        debug!("Authenticating a Google user.");

        let userinfo = self.get_google_userinfo(req.token).await?;
        let user = self.get_user(&userinfo).await?;

        let token = self.tokens.encode(&TokenClaims {
            exp: get_timestamp() + TOKEN_TTL,
            sub: user.id.to_string(),
        })?;

        Ok(LoginResponse {
            token,
            name: user.name,
            picture: user.picture,
        })
    }

    async fn get_user(&self, userinfo: &GoogleUserinfoResponse) -> Result<UserRecord> {
        if let Some(user) = self.users.get_by_email(&userinfo.email).await? {
            debug!("Found a user with email {}.", userinfo.email);
            return Ok(user);
        }

        debug!("Creating a new user with email {}.", userinfo.email);

        let user = UserRecord {
            id: get_unique_id()?,
            email: userinfo.email.clone(),
            name: userinfo.name.clone(),
            picture: userinfo.picture.clone(),
            ..Default::default()
        };

        self.users.add(&user).await?;

        info!(
            "Created a new user with email {} and id {}.",
            user.email, user.id
        );

        Ok(user)
    }

    async fn get_google_userinfo(&self, access_token: String) -> Result<GoogleUserinfoResponse> {
        let client = reqwest::Client::new();
        let headers = self.get_login_headers(&access_token)?;

        let req = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .headers(headers);

        debug!("Request: {req:?}");

        let res = match req.send().await {
            Ok(res) => res,

            Err(e) => {
                debug!("Failed to get user info from Google: {e:?}");
                return Err(Error::GoogleUserInfo);
            }
        };

        match res.json::<GoogleUserinfoResponse>().await {
            Ok(u) => Ok(u),

            Err(e) => {
                debug!("Failed to parse Google response: {e:?}");
                Err(Error::GoogleUserInfo)
            }
        }
    }

    fn get_login_headers(&self, token: &str) -> Result<HeaderMap> {
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

impl Locatable for LoginGoogleHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
            tokens: locator.get::<TokenService>()?,
        })
    }
}
