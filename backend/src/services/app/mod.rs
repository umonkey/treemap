use crate::handlers::*;
use crate::services::Locator;
use crate::services::{FileService, TokenService};
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub struct AppState {
    files: Arc<FileService>,
    tokens: Arc<TokenService>,
    pub add_comment_handler: Arc<AddCommentHandler>,
    pub add_trees_handler: Arc<AddTreesHandler>,
    pub get_file_status_handler: Arc<GetFileStatusHandler>,
    pub get_me_handler: Arc<GetMeHandler>,
    pub get_new_comments_handler: Arc<GetNewCommentsHandler>,
    pub get_new_trees_handler: Arc<GetNewTreesHandler>,
    pub get_tree_comments_handler: Arc<GetTreeCommentsHandler>,
    pub get_tree_defaults_handler: Arc<GetTreeDefaultsHandler>,
    pub get_tree_handler: Arc<GetTreeHandler>,
    pub get_tree_stats_handler: Arc<GetTreeStatsHandler>,
    pub get_trees_handler: Arc<GetTreesHandler>,
    pub get_updated_trees_handler: Arc<GetUpdatedTreesHandler>,
    pub get_user_handler: Arc<GetUserHandler>,
    pub like_tree_handler: Arc<LikeTreeHandler>,
    pub login_google_handler: Arc<LoginGoogleHandler>,
    pub login_google_v2_handler: Arc<LoginGoogleV2Handler>,
    pub login_google_v3_handler: Arc<LoginGoogleV3Handler>,
    pub move_tree_handler: Arc<MoveTreeHandler>,
    pub search_species_handler: Arc<SearchSpeciesHandler>,
    pub suggest_species_handler: Arc<SuggestSpeciesHandler>,
    pub unlike_tree_handler: Arc<UnlikeTreeHandler>,
    pub update_tree_handler: Arc<UpdateTreeHandler>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let locator = Locator::new();

        Ok(Self {
            files: locator.get::<FileService>()?,
            tokens: locator.get::<TokenService>()?,
            add_comment_handler: locator.get::<AddCommentHandler>()?,
            add_trees_handler: locator.get::<AddTreesHandler>()?,
            get_file_status_handler: locator.get::<GetFileStatusHandler>()?,
            get_me_handler: locator.get::<GetMeHandler>()?,
            get_new_comments_handler: locator.get::<GetNewCommentsHandler>()?,
            get_new_trees_handler: locator.get::<GetNewTreesHandler>()?,
            get_tree_comments_handler: locator.get::<GetTreeCommentsHandler>()?,
            get_tree_defaults_handler: locator.get::<GetTreeDefaultsHandler>()?,
            get_tree_handler: locator.get::<GetTreeHandler>()?,
            get_tree_stats_handler: locator.get::<GetTreeStatsHandler>()?,
            get_trees_handler: locator.get::<GetTreesHandler>()?,
            get_updated_trees_handler: locator.get::<GetUpdatedTreesHandler>()?,
            get_user_handler: locator.get::<GetUserHandler>()?,
            like_tree_handler: locator.get::<LikeTreeHandler>()?,
            login_google_handler: locator.get::<LoginGoogleHandler>()?,
            login_google_v2_handler: locator.get::<LoginGoogleV2Handler>()?,
            login_google_v3_handler: locator.get::<LoginGoogleV3Handler>()?,
            move_tree_handler: locator.get::<MoveTreeHandler>()?,
            search_species_handler: locator.get::<SearchSpeciesHandler>()?,
            suggest_species_handler: locator.get::<SuggestSpeciesHandler>()?,
            unlike_tree_handler: locator.get::<UnlikeTreeHandler>()?,
            update_tree_handler: locator.get::<UpdateTreeHandler>()?,
        })
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

    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        self.files.get_file(id).await
    }

    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileUploadResponse> {
        let file = self.files.add_file(req).await?;
        Ok(FileUploadResponse::from_file(&file))
    }
}
