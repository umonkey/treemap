use crate::handlers::*;
use crate::services::database::get_database;
use crate::services::trees::Trees;
use crate::services::Locator;
use crate::services::{get_file_storage, Database, FileService, GoogleAuth, TokenService};
use crate::types::*;
use actix_web::HttpRequest;
use log::info;
use std::sync::Arc;

pub struct AppState {
    db: Arc<dyn Database>,
    files: FileService,
    gauth: GoogleAuth,
    tokens: TokenService,
    trees: Trees,
    pub add_comment_handler: Arc<AddCommentHandler>,
    pub add_trees_handler: Arc<AddTreesHandler>,
    pub get_me_handler: Arc<GetMeHandler>,
    pub get_new_comments_handler: Arc<GetNewCommentsHandler>,
    pub get_new_trees_handler: Arc<GetNewTreesHandler>,
    pub get_tree_comments_handler: Arc<GetTreeCommentsHandler>,
    pub get_tree_defaults_handler: Arc<GetTreeDefaultsHandler>,
    pub get_tree_handler: Arc<GetTreeHandler>,
    pub get_updated_trees_handler: Arc<GetUpdatedTreesHandler>,
    pub get_user_handler: Arc<GetUserHandler>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let locator = Locator::new();

        let db = get_database().await?;
        let token = TokenService::new();
        let storage = get_file_storage().await?;

        Ok(Self {
            db: db.clone(),
            files: FileService::new(&db, &storage)?,
            gauth: GoogleAuth::new(&db, &token).await,
            tokens: token,
            trees: Trees::new(&db).await,
            add_comment_handler: locator.get::<AddCommentHandler>()?,
            add_trees_handler: locator.get::<AddTreesHandler>()?,
            get_me_handler: locator.get::<GetMeHandler>()?,
            get_new_comments_handler: locator.get::<GetNewCommentsHandler>()?,
            get_new_trees_handler: locator.get::<GetNewTreesHandler>()?,
            get_tree_comments_handler: locator.get::<GetTreeCommentsHandler>()?,
            get_tree_defaults_handler: locator.get::<GetTreeDefaultsHandler>()?,
            get_tree_handler: locator.get::<GetTreeHandler>()?,
            get_updated_trees_handler: locator.get::<GetUpdatedTreesHandler>()?,
            get_user_handler: locator.get::<GetUserHandler>()?,
        })
    }

    pub async fn update_tree(&self, req: UpdateTreeRequest) -> Result<TreeRecord> {
        self.trees.update_tree(req).await
    }

    pub async fn move_tree(&self, req: MoveTreeRequest) -> Result<()> {
        self.trees.move_tree(&req).await?;

        info!("Tree {} moved to ({},{})", req.id, req.lat, req.lon);

        Ok(())
    }

    pub async fn get_trees(&self, request: &GetTreesRequest) -> Result<TreeList> {
        self.trees.get_trees(request).await
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

    pub async fn get_tree_stats(&self) -> Result<TreeStats> {
        self.trees.get_tree_stats().await
    }

    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        self.files.get_file(id).await
    }

    pub async fn get_file_status(&self, id: u64) -> Result<FileStatusResponse> {
        self.files.get_status(id).await
    }

    /**
     * Deprecated.
     */
    pub async fn login_google(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        self.gauth.login(req).await
    }

    /**
     * Use the new signin API.
     */
    pub async fn login_google_v2(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        self.gauth.login_v2(req).await
    }

    /**
     * Use the new signin API.
     */
    pub async fn login_google_v3(&self, req: GoogleAuthCallbackPayload) -> Result<String> {
        self.gauth.login_v3(req).await
    }

    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileUploadResponse> {
        let file = self.files.add_file(req).await?;
        Ok(FileUploadResponse::from_file(&file))
    }

    pub async fn find_species(&self, query: &str) -> Result<Vec<PublicSpeciesInfo>> {
        let records = self.db.find_species(query).await?;
        let species = records.iter().map(PublicSpeciesInfo::from_record).collect();
        Ok(species)
    }

    pub async fn suggest_species(&self, user_id: u64) -> Result<Vec<String>> {
        self.db.find_recent_species(user_id).await
    }

    pub async fn like_tree(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.db.like_tree(tree_id, user_id).await?;
        Ok(())
    }

    pub async fn unlike_tree(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.db.unlike_tree(tree_id, user_id).await?;
        Ok(())
    }
}
