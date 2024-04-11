use actix_web::HttpRequest;

use crate::Result;
use crate::services::{GoogleAuth, TokenService};
use crate::services::database::get_database;
use crate::services::trees::Trees;
use crate::types::{AddTreeRequest, Bounds, TreeInfo, TreeList, LoginGoogleRequest, LoginResponse, TreeDetails};
use crate::errors::Error;

pub struct AppState {
    trees: Trees,
    gauth: GoogleAuth,
    tokens: TokenService,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let db = get_database().await?;

        Ok(Self {
            trees: Trees::init(&db).await,
            gauth: GoogleAuth::init(&db).await,
            tokens: TokenService::new(),
        })
    }

    pub async fn add_tree(&self, req: AddTreeRequest) -> Result<TreeInfo> {
        self.trees.add_tree(req).await
    }

    pub async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        self.trees.get_trees(bounds).await
    }

    pub fn get_user_id(&self, req: &HttpRequest) -> Result<u64> {
        let header = match req.headers().get("Authorization") {
            Some(h) => h,
            None => return Err(Error::MissingAuthorizationHeader),
        };

        let value = match header.to_str() {
            Ok(v) => v,
            Err(_) => return Err(Error::BadAuthorizationHeader),
        };

        let payload = match value.strip_prefix("Bearer ") {
            Some(p) => p,
            None => return Err(Error::BadAuthorizationHeader),
        };

        let token = match self.tokens.decode(payload) {
            Ok(t) => t,
            Err(_) => return Err(Error::BadAuthToken),
        };

        let user_id: u64 = match token.sub.parse() {
            Ok(id) => id,
            Err(_) => return Err(Error::BadAuthToken),
        };

        // TODO: check if user exists.

        Ok(user_id)
    }

    pub async fn get_tree(&self, id: u64) -> Result<TreeDetails> {
        let tree = self.trees.get_tree(id).await?;
        Ok(TreeDetails::from_tree(&tree))
    }

    pub async fn login_google(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        self.gauth.login(req).await
    }
}
