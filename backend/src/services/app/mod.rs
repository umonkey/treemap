use crate::common::database::repositories::*;
use crate::handlers::*;
use crate::services::Locator;
use crate::services::{FileService, TokenService};
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub struct AppState {
    files: Arc<FileService>,
    users: Arc<UserRepository>,
    tokens: Arc<TokenService>,
    pub add_comment_handler: Arc<AddCommentHandler>,
    pub add_training_handler: Arc<AddTrainingHandler>,
    pub add_trees_handler: Arc<AddTreesHandler>,
    pub delete_file_handler: Arc<DeleteFileHandler>,
    pub get_file_status_handler: Arc<GetFileStatusHandler>,
    pub get_me_handler: Arc<GetMeHandler>,
    pub get_new_comments_handler: Arc<GetNewCommentsHandler>,
    pub get_new_trees_handler: Arc<GetNewTreesHandler>,
    pub get_species_mismatch_handler: Arc<GetSpeciesMismatchHandler>,
    pub get_species_stats_handler: Arc<GetSpeciesStatsHandler>,
    pub get_top_circumference_handler: Arc<GetTopCircumferenceHandler>,
    pub get_top_diameter_handler: Arc<GetTopDiameterHandler>,
    pub get_top_height_handler: Arc<GetTopHeightHandler>,
    pub get_top_streets_handler: Arc<GetTopStreetsHandler>,
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
    pub login_osm_handler: Arc<LoginOsmHandler>,
    pub move_tree_handler: Arc<MoveTreeHandler>,
    pub search_species_handler: Arc<SearchSpeciesHandler>,
    pub suggest_species_handler: Arc<SuggestSpeciesHandler>,
    pub unlike_tree_handler: Arc<UnlikeTreeHandler>,
    pub update_tree_handler: Arc<UpdateTreeHandler>,
    pub update_tree_height_handler: Arc<UpdateTreeHeightHandler>,
    pub update_tree_diameter_handler: Arc<UpdateTreeDiameterHandler>,
    pub update_tree_circumference_handler: Arc<UpdateTreeCircumferenceHandler>,
    pub update_tree_thumbnail_handler: Arc<UpdateTreeThumbnailHandler>,
    pub update_tree_state_handler: Arc<UpdateTreeStateHandler>,
}

impl AppState {
    pub async fn new(locator: Arc<Locator>) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileService>()?,
            users: locator.get::<UserRepository>()?,
            tokens: locator.get::<TokenService>()?,
            add_comment_handler: locator.get::<AddCommentHandler>()?,
            add_training_handler: locator.get::<AddTrainingHandler>()?,
            add_trees_handler: locator.get::<AddTreesHandler>()?,
            delete_file_handler: locator.get::<DeleteFileHandler>()?,
            get_file_status_handler: locator.get::<GetFileStatusHandler>()?,
            get_me_handler: locator.get::<GetMeHandler>()?,
            get_new_comments_handler: locator.get::<GetNewCommentsHandler>()?,
            get_new_trees_handler: locator.get::<GetNewTreesHandler>()?,
            get_species_mismatch_handler: locator.get::<GetSpeciesMismatchHandler>()?,
            get_species_stats_handler: locator.get::<GetSpeciesStatsHandler>()?,
            get_top_circumference_handler: locator.get::<GetTopCircumferenceHandler>()?,
            get_top_diameter_handler: locator.get::<GetTopDiameterHandler>()?,
            get_top_height_handler: locator.get::<GetTopHeightHandler>()?,
            get_top_streets_handler: locator.get::<GetTopStreetsHandler>()?,
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
            login_osm_handler: locator.get::<LoginOsmHandler>()?,
            move_tree_handler: locator.get::<MoveTreeHandler>()?,
            search_species_handler: locator.get::<SearchSpeciesHandler>()?,
            suggest_species_handler: locator.get::<SuggestSpeciesHandler>()?,
            unlike_tree_handler: locator.get::<UnlikeTreeHandler>()?,
            update_tree_handler: locator.get::<UpdateTreeHandler>()?,
            update_tree_height_handler: locator.get::<UpdateTreeHeightHandler>()?,
            update_tree_diameter_handler: locator.get::<UpdateTreeDiameterHandler>()?,
            update_tree_circumference_handler: locator.get::<UpdateTreeCircumferenceHandler>()?,
            update_tree_thumbnail_handler: locator.get::<UpdateTreeThumbnailHandler>()?,
            update_tree_state_handler: locator.get::<UpdateTreeStateHandler>()?,
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
        let user_id = req.user_id;
        let file = self.files.add_file(req).await?;
        self.users.increment_files_count(user_id, 1).await?;
        Ok(FileUploadResponse::from_file(&file))
    }
}
