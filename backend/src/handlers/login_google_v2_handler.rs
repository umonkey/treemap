use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use jsonwebtoken::{decode, decode_header, Algorithm, TokenData, Validation};
use jwks::Jwks;
use log::{debug, error, info};
use std::sync::Arc;

const TOKEN_TTL: u64 = 30 * 86400; // 30 days

pub struct LoginGoogleV2Handler {
    db: Arc<dyn Database + Send + Sync>,
    tokens: Arc<TokenService>,
}

impl LoginGoogleV2Handler {
    pub fn new(db: Arc<SqliteDatabase>, tokens: Arc<TokenService>) -> Self {
        Self { db, tokens }
    }

    pub async fn handle(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
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
}

impl Locatable for LoginGoogleV2Handler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        let tokens = locator.get::<TokenService>()?;
        Ok(Self::new(db, tokens))
    }
}
