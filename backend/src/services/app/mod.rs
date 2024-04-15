use actix_web::HttpRequest;
use log::info;

use crate::errors::Error;
use crate::services::database::get_database;
use crate::services::trees::Trees;
use crate::services::{FileService, GoogleAuth, TokenService, UploadService};
use crate::types::{
    AddTreeRequest, Bounds, LoginGoogleRequest, LoginResponse, MoveTreeRequest, TreeDetails, TreeInfo, TreeList,
    UpdateTreeRequest, UploadTicket, AddFileRequest, FileRecord,
};
use crate::Result;

pub struct AppState {
    files: FileService,
    gauth: GoogleAuth,
    tokens: TokenService,
    trees: Trees,
    uploads: UploadService,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let db = get_database().await?;
        let token = TokenService::new();

        Ok(Self {
            files: FileService::init(&db)?,
            gauth: GoogleAuth::init(&db, &token).await,
            tokens: token,
            trees: Trees::init(&db).await,
            uploads: UploadService::init(&db).await?,
        })
    }

    pub async fn add_tree(&self, req: AddTreeRequest) -> Result<TreeInfo> {
        self.trees.add_tree(req).await
    }

    pub async fn update_tree(&self, req: UpdateTreeRequest) -> Result<TreeInfo> {
        self.trees.update_tree(req).await
    }

    pub async fn move_tree(&self, req: MoveTreeRequest) -> Result<()> {
        self.trees.move_tree(&req).await?;

        info!("Tree {} moved to ({},{})", req.id, req.lat, req.lon);

        Ok(())
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

    /**
     * Creates an upload ticket for the specified user.
     */
    pub async fn create_upload_ticket(&self, user_id: u64) -> Result<UploadTicket> {
        self.uploads.create_ticket(user_id).await
    }

    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileRecord> {
        self.files.add_file(req).await
    }
}
