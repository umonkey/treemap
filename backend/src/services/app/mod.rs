use actix_web::HttpRequest;
use log::info;
use std::sync::Arc;

use crate::services::database::get_database;
use crate::services::trees::Trees;
use crate::services::{
    CommentsService, Database, FileService, GoogleAuth, TokenService, UploadService,
};
use crate::types::{
    AddCommentRequest, AddFileRequest, AddTreeRequest, Error, FileRecord, GetTreesRequest,
    LoginGoogleRequest, LoginResponse, MoveTreeRequest, PublicCommentInfo, PublicSpeciesInfo,
    Result, TreeDetails, TreeList, TreeRecord, UpdateTreeRequest, UploadTicketRecord,
    MeResponse,
};

pub struct AppState {
    db: Arc<dyn Database>,
    comments: CommentsService,
    files: FileService,
    gauth: GoogleAuth,
    tokens: TokenService,
    trees: Trees,
    uploads: UploadService,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let db = get_database().await?;
        let token = TokenService::new();

        Ok(Self {
            db: db.clone(),
            comments: CommentsService::new(&db),
            files: FileService::new(&db)?,
            gauth: GoogleAuth::new(&db, &token).await,
            tokens: token,
            trees: Trees::new(&db).await,
            uploads: UploadService::new(&db).await?,
        })
    }

    pub async fn add_tree(&self, req: AddTreeRequest) -> Result<TreeRecord> {
        self.trees.add_tree(req).await
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

    pub async fn get_tree(&self, id: u64) -> Result<TreeDetails> {
        let tree = self.trees.get_tree(id).await?;
        let files = self.files.find_files_by_tree(id).await?;

        Ok(TreeDetails::from_tree(&tree, &files))
    }

    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        self.files.get_file(id).await
    }

    pub async fn login_google(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        self.gauth.login(req).await
    }

    pub async fn get_user_info(&self, user_id: u64) -> Result<MeResponse> {
        let user = match self.db.get_user(user_id).await? {
            Some(u) => u,
            None => return Err(Error::UserNotFound),
        };

        Ok(MeResponse {
            name: user.name,
            picture: user.picture,
        })
    }

    /**
     * Creates an upload ticket for the specified user.
     */
    pub async fn create_upload_ticket(&self, user_id: u64) -> Result<UploadTicketRecord> {
        self.uploads.create_ticket(user_id).await
    }

    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileRecord> {
        self.files.add_file(req).await
    }

    pub async fn add_comment(&self, req: AddCommentRequest) -> Result<()> {
        self.comments.add_comment(&req).await
    }

    pub async fn get_comments(&self, tree_id: u64) -> Result<Vec<PublicCommentInfo>> {
        let records = self.comments.get_comments(tree_id).await?;
        let comments = records.iter().map(PublicCommentInfo::from_record).collect();
        Ok(comments)
    }

    pub async fn find_species(&self, query: &str) -> Result<Vec<PublicSpeciesInfo>> {
        let records = self.db.find_species(query).await?;
        let species = records.iter().map(PublicSpeciesInfo::from_record).collect();
        Ok(species)
    }

    pub async fn suggest_species(&self, user_id: u64) -> Result<Vec<String>> {
        self.db.find_recent_species(user_id).await
    }
}
