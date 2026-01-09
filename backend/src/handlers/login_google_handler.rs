use crate::common::database::repositories::*;
use crate::infra::google_auth::{AuthResponse, GoogleAuthClient};
use crate::infra::tokens::{TokenClaims, TokenService};
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;

const TOKEN_TTL: u64 = 30 * 86400; // 30 days

pub struct LoginGoogleHandler {
    users: Arc<UserRepository>,
    tokens: Arc<TokenService>,
    auth: GoogleAuthClient,
}

impl LoginGoogleHandler {
    pub async fn handle(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        debug!("Authenticating a Google user.");

        let userinfo = self.auth.verify_token(&req.token).await?;
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

    async fn get_user(&self, userinfo: &AuthResponse) -> Result<User> {
        if let Some(user) = self.users.get_by_email(&userinfo.email).await? {
            debug!("Found a user with email {}.", userinfo.email);
            return Ok(user);
        }

        debug!("Creating a new user with email {}.", userinfo.email);

        let user = User {
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
}

impl Locatable for LoginGoogleHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
            tokens: locator.get::<TokenService>()?,
            auth: GoogleAuthClient::new(),
        })
    }
}
