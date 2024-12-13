use jsonwebtoken::{decode, decode_header, Algorithm, TokenData, Validation};
use jwks::Jwks;
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::sync::Arc;

use crate::services::{Database, TokenService};
use crate::types::{
    Error, GoogleUserinfoResponse, LoginGoogleRequest, LoginResponse, Result, TokenClaims,
    UserRecord,
};
use crate::utils::{get_timestamp, get_unique_id};

const TOKEN_TTL: u64 = 30 * 86400; // 30 days

#[derive(Debug, Deserialize)]
pub struct GoogleIdToken {
    pub email: String,
    pub name: String,
    pub picture: String,
}

pub struct GoogleAuth {
    db: Arc<dyn Database>,
    tokens: TokenService,
}

impl GoogleAuth {
    pub async fn new(db: &Arc<dyn Database>, tokens: &TokenService) -> Self {
        Self {
            db: db.clone(),
            tokens: tokens.clone(),
        }
    }

    pub async fn login(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        debug!("Authenticating a Google user.");

        let userinfo = self.get_google_userinfo(req).await?;
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

    pub async fn login_v2(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        debug!("Authenticating a Google user (v2).");

        let header = decode_header(&req.token)?;
        let kid = header.kid.as_ref().ok_or(Error::BadAuthToken)?;

        let jwks_url = "https://www.googleapis.com/oauth2/v3/certs";

        let jwks = match Jwks::from_jwks_url(jwks_url).await {
            Ok(value) => value,
            Err(e) => {
                error!("Error fetching JWKS: {:?}", e);
                return Err(Error::BadAuthToken);
            }
        };

        let jwk = match jwks.keys.get(kid).ok_or(Error::BadAuthToken) {
            Ok(value) => value,
            Err(e) => {
                error!("Error fetching JWKS key: {:?}", e);
                return Err(Error::BadAuthToken);
            }
        };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_aud = false;

        let decoded_token: TokenData<GoogleIdToken> =
            match decode::<GoogleIdToken>(&req.token, &jwk.decoding_key, &validation) {
                Ok(value) => value,
                Err(e) => {
                    error!("Error decoding token: {:?}", e);
                    return Err(Error::BadAuthToken);
                }
            };

        let user_info = GoogleUserinfoResponse {
            id: "unused".to_string(),
            email: decoded_token.claims.email.clone(),
            verified_email: true,
            name: decoded_token.claims.name.clone(),
            picture: decoded_token.claims.picture.clone(),
            locale: None,
        };

        let user = self.get_user(&user_info).await?;

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
        if let Some(user) = self.db.find_user_by_email(&userinfo.email).await? {
            debug!("Found a user with email {}.", userinfo.email);
            return Ok(user);
        }

        debug!("Creating a new user with email {}.", userinfo.email);

        let user = UserRecord {
            id: get_unique_id()?,
            email: userinfo.email.clone(),
            name: userinfo.name.clone(),
            picture: userinfo.picture.clone(),
        };

        self.db.add_user(&user).await?;

        info!(
            "Created a new user with email {} and id {}.",
            user.email, user.id
        );

        Ok(user)
    }

    async fn get_google_userinfo(&self, req: LoginGoogleRequest) -> Result<GoogleUserinfoResponse> {
        let client = reqwest::Client::new();

        let headers = match self.get_login_headers(&req) {
            Ok(value) => value,

            Err(e) => {
                debug!("Failed to get login headers: {:?}", e);
                return Err(Error::GoogleUserInfo);
            }
        };

        let req = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .headers(headers);

        debug!("Request: {:?}", req);

        let res = match req.send().await {
            Ok(res) => res,

            Err(e) => {
                debug!("Failed to get user info from Google: {:?}", e);
                return Err(Error::GoogleUserInfo);
            }
        };

        match res.json::<GoogleUserinfoResponse>().await {
            Ok(u) => Ok(u),

            Err(e) => {
                debug!("Failed to parse Google response: {:?}", e);
                Err(Error::GoogleUserInfo)
            }
        }
    }

    fn get_login_headers(&self, req: &LoginGoogleRequest) -> Result<HeaderMap> {
        let auth_header = match HeaderValue::from_str(format!("Bearer {}", req.token).as_str()) {
            Ok(h) => h,

            Err(e) => {
                debug!("Failed to create auth header: {}", e);
                return Err(Error::BadAuthToken);
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", auth_header);
        Ok(headers)
    }
}
