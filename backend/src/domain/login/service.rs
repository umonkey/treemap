use super::schemas::*;
use crate::domain::user::{User, UserRepository};
use crate::infra::google_auth::{AuthResponse, GoogleAuthClient};
use crate::infra::osm::OsmClient;
use crate::infra::tokens::{TokenClaims, TokenService};
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;
use url::Url;

const TOKEN_TTL: u64 = 30 * 86400; // 30 days

pub struct LoginService {
    tokens: Arc<TokenService>,
    users: Arc<UserRepository>,
    auth: GoogleAuthClient,
    osm: Arc<OsmClient>,
}

impl LoginService {
    pub async fn login_google(&self, req: GoogleAuthCallbackPayload) -> Result<String> {
        debug!("Authenticating a Google user (v3).");

        let userinfo = self.auth.verify_token(&req.access_token).await?;
        let user = self.get_user(&userinfo).await?;

        let token = self.tokens.encode(&TokenClaims {
            exp: get_timestamp() + TOKEN_TTL,
            sub: user.id.to_string(),
        })?;

        let redirect = self.get_redirect_url(&req.state, &token)?;
        Ok(redirect)
    }

    pub async fn login_osm(&self, code: String) -> Result<()> {
        debug!("Authenticating an OSM user.");

        let token = self.osm.get_token(&code).await?;

        info!("OSM token: {token}");

        Ok(())
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
            role: "user".to_string(),
            ..Default::default()
        };

        self.users.add(&user).await?;

        info!(
            "Created a new user with email {} and id {}.",
            user.email, user.id
        );

        Ok(user)
    }

    fn get_redirect_url(&self, target: &str, token: &str) -> Result<String> {
        let origin = Url::parse(target)
            .map_err(|_| Error::BadCallback)?
            .origin()
            .unicode_serialization();

        debug!("Auth callback: origin={origin}, token={token}, target={target}");

        let callback = format!("{origin}/auth?token={token}&state={target}");
        Ok(callback)
    }
}

impl Locatable for LoginService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            tokens: locator.get::<TokenService>()?,
            users: locator.get::<UserRepository>()?,
            auth: GoogleAuthClient::new(),
            osm: locator.get::<OsmClient>()?,
        })
    }
}
