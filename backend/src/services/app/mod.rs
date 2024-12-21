use crate::handlers::*;
use crate::services::database::get_database;
use crate::services::trees::Trees;
use crate::services::Locator;
use crate::services::{
    get_file_storage, CommentsService, Database, FileService, GoogleAuth, TokenService,
};
use crate::types::{
    AddFileRequest, AddTreeRequest, CommentList, CommentRecord, Error, FileRecord,
    FileStatusResponse, FileUploadResponse, GetTreesRequest, GoogleAuthCallbackPayload,
    LoginGoogleRequest, LoginResponse, MeResponse, MoveTreeRequest, NewTreeDefaultsResponse,
    PublicSpeciesInfo, Result, TreeDetails, TreeList, TreeRecord, TreeStats, UpdateTreeRequest,
    UserRecord, UserResponse,
};
use actix_web::HttpRequest;
use log::info;
use std::collections::HashSet;
use std::sync::Arc;

pub struct AppState {
    db: Arc<dyn Database>,
    comments: CommentsService,
    files: FileService,
    gauth: GoogleAuth,
    tokens: TokenService,
    trees: Trees,
    pub add_comment_handler: Arc<AddCommentHandler>,
    pub get_new_trees_handler: Arc<GetNewTreesHandler>,
    pub get_new_comments_handler: Arc<GetNewCommentsHandler>,
    pub get_updated_trees_handler: Arc<GetUpdatedTreesHandler>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let locator = Locator::new();

        let db = get_database().await?;
        let token = TokenService::new();
        let storage = get_file_storage().await?;

        Ok(Self {
            db: db.clone(),
            comments: CommentsService::new(&db),
            files: FileService::new(&db, &storage)?,
            gauth: GoogleAuth::new(&db, &token).await,
            tokens: token,
            trees: Trees::new(&db).await,
            add_comment_handler: locator.get::<AddCommentHandler>()?,
            get_new_trees_handler: locator.get::<GetNewTreesHandler>()?,
            get_new_comments_handler: locator.get::<GetNewCommentsHandler>()?,
            get_updated_trees_handler: locator.get::<GetUpdatedTreesHandler>()?,
        })
    }

    pub async fn add_trees(&self, req: AddTreeRequest) -> Result<Vec<TreeRecord>> {
        self.trees.add_trees(req).await
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

    pub async fn get_user(&self, id: u64) -> Result<UserResponse> {
        let record = match self.db.get_user(id).await? {
            Some(value) => value,

            None => return Err(Error::UserNotFound),
        };

        Ok(UserResponse::from(record))
    }

    pub async fn get_tree_defaults(&self, user_id: u64) -> Result<NewTreeDefaultsResponse> {
        match self.trees.get_last_tree_by_user(user_id).await? {
            Some(tree) => Ok(NewTreeDefaultsResponse::from_tree(&tree)),

            None => Ok(NewTreeDefaultsResponse {
                species: None,
                notes: None,
                height: Some(0.0),
                circumference: Some(0.0),
                diameter: Some(0.0),
                state: Some("healthy".to_string()),
            }),
        }
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
        let users = self.collect_users(&tree, &files).await?;

        Ok(TreeDetails::from_tree(&tree, &files, &users))
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

    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileUploadResponse> {
        let file = self.files.add_file(req).await?;
        Ok(FileUploadResponse::from_file(&file))
    }

    pub async fn get_tree_comments(&self, tree_id: u64) -> Result<CommentList> {
        let records = self.comments.get_tree_comments(tree_id).await?;
        self.format_comment_list(&records).await
    }

    async fn format_comment_list(&self, comments: &[CommentRecord]) -> Result<CommentList> {
        let user_ids: Vec<u64> = comments.iter().map(|r| r.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        let tree_ids: Vec<u64> = comments.iter().map(|r| r.tree_id).collect();
        let trees = self.load_trees(&tree_ids).await?;

        Ok(CommentList::from_records(comments, &users, &trees))
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

    async fn collect_users(
        &self,
        tree: &TreeRecord,
        files: &Vec<FileRecord>,
    ) -> Result<Vec<UserResponse>> {
        let mut user_ids = Vec::new();

        user_ids.push(tree.added_by);

        for file in files {
            user_ids.push(file.added_by);
        }

        let users = self.load_users(&user_ids).await?;
        let users = users.iter().map(UserResponse::from).collect();

        Ok(users)
    }

    async fn load_users(&self, user_ids: &[u64]) -> Result<Vec<UserRecord>> {
        let user_ids = HashSet::<u64>::from_iter(user_ids.iter().copied());

        let mut users = Vec::new();

        for id in user_ids {
            if let Ok(Some(user)) = self.db.get_user(id).await {
                users.push(user);
            };
        }

        Ok(users)
    }

    async fn load_trees(&self, tree_ids: &[u64]) -> Result<Vec<TreeRecord>> {
        let tree_ids = HashSet::<u64>::from_iter(tree_ids.iter().copied());

        let mut trees = Vec::new();

        for id in tree_ids {
            if let Ok(Some(tree)) = self.db.get_tree(id).await {
                trees.push(tree);
            };
        }

        Ok(trees)
    }
}
