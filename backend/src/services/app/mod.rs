use crate::domain::file::FileService;
use crate::domain::health::*;
use crate::domain::heatmap::HeatmapService;
use crate::domain::training::TrainingService;
use crate::domain::user::*;
use crate::handlers::*;
use crate::infra::tokens::TokenService;
use crate::services::Locator;
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub struct AppState {
    pub files: Arc<FileService>,
    tokens: Arc<TokenService>,
    pub user_service: Arc<UserService>,
    pub add_comment_handler: Arc<AddCommentHandler>,
    pub add_photos_handler: Arc<AddPhotosHandler>,
    pub training: Arc<TrainingService>,
    pub add_trees_handler: Arc<AddTreesHandler>,
    pub get_duplicates_handler: Arc<GetDuplicatesHandler>,
    pub get_health_handler: Arc<GetHealthHandler>,
    pub heatmap: Arc<HeatmapService>,
    pub get_me_handler: Arc<GetMeHandler>,
    pub get_me_likes_handler: Arc<GetMeLikesHandler>,
    pub get_new_comments_handler: Arc<GetNewCommentsHandler>,
    pub get_new_trees_handler: Arc<GetNewTreesHandler>,
    pub get_species_mismatch_handler: Arc<GetSpeciesMismatchHandler>,
    pub get_species_stats_handler: Arc<GetSpeciesStatsHandler>,
    pub get_state_stats_handler: Arc<GetStateStatsHandler>,
    pub get_street_report_handler: Arc<GetStreetReportHandler>,
    pub get_top_circumference_handler: Arc<GetTopCircumferenceHandler>,
    pub get_top_diameter_handler: Arc<GetTopDiameterHandler>,
    pub get_top_height_handler: Arc<GetTopHeightHandler>,
    pub get_top_streets_handler: Arc<GetTopStreetsHandler>,
    pub get_tree_comments_handler: Arc<GetTreeCommentsHandler>,
    pub get_tree_defaults_handler: Arc<GetTreeDefaultsHandler>,
    pub get_tree_handler: Arc<GetTreeHandler>,
    pub get_tree_history_handler: Arc<GetTreeHistoryHandler>,
    pub get_tree_stats_handler: Arc<GetTreeStatsHandler>,
    pub get_trees_handler: Arc<GetTreesHandler>,
    pub get_updated_trees_handler: Arc<GetUpdatedTreesHandler>,
    pub like_tree_handler: Arc<LikeTreeHandler>,
    pub login_google_v3_handler: Arc<LoginGoogleV3Handler>,
    pub login_osm_handler: Arc<LoginOsmHandler>,
    pub move_tree_handler: Arc<MoveTreeHandler>,
    pub replace_tree_handler: Arc<ReplaceTreeHandler>,
    pub search_species_handler: Arc<SearchSpeciesHandler>,
    pub search_streets_handler: Arc<SearchStreetsHandler>,
    pub suggest_species_handler: Arc<SuggestSpeciesHandler>,
    pub tree_page_handler: Arc<TreePageHandler>,
    pub unlike_tree_handler: Arc<UnlikeTreeHandler>,
    pub update_settings_handler: Arc<UpdateSettingsHandler>,
    pub update_tree_handler: Arc<UpdateTreeHandler>,
    pub update_tree_height_handler: Arc<UpdateTreeHeightHandler>,
    pub update_tree_location_handler: Arc<UpdateTreeLocationHandler>,
    pub update_tree_diameter_handler: Arc<UpdateTreeDiameterHandler>,
    pub update_tree_circumference_handler: Arc<UpdateTreeCircumferenceHandler>,
    pub update_tree_thumbnail_handler: Arc<UpdateTreeThumbnailHandler>,
    pub update_tree_state_handler: Arc<UpdateTreeStateHandler>,
    pub upload_handler: Arc<UploadHandler>,
}

impl AppState {
    pub async fn new(locator: Arc<Locator>) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileService>()?,
            tokens: locator.get::<TokenService>()?,
            user_service: locator.get::<UserService>()?,
            add_comment_handler: locator.get::<AddCommentHandler>()?,
            add_photos_handler: locator.get::<AddPhotosHandler>()?,
            training: locator.get::<TrainingService>()?,
            add_trees_handler: locator.get::<AddTreesHandler>()?,
            get_duplicates_handler: locator.get::<GetDuplicatesHandler>()?,
            get_health_handler: locator.get::<GetHealthHandler>()?,
            heatmap: locator.get::<HeatmapService>()?,
            get_me_handler: locator.get::<GetMeHandler>()?,
            get_me_likes_handler: locator.get::<GetMeLikesHandler>()?,
            get_new_comments_handler: locator.get::<GetNewCommentsHandler>()?,
            get_new_trees_handler: locator.get::<GetNewTreesHandler>()?,
            get_species_mismatch_handler: locator.get::<GetSpeciesMismatchHandler>()?,
            get_species_stats_handler: locator.get::<GetSpeciesStatsHandler>()?,
            get_state_stats_handler: locator.get::<GetStateStatsHandler>()?,
            get_street_report_handler: locator.get::<GetStreetReportHandler>()?,
            get_top_circumference_handler: locator.get::<GetTopCircumferenceHandler>()?,
            get_top_diameter_handler: locator.get::<GetTopDiameterHandler>()?,
            get_top_height_handler: locator.get::<GetTopHeightHandler>()?,
            get_top_streets_handler: locator.get::<GetTopStreetsHandler>()?,
            get_tree_comments_handler: locator.get::<GetTreeCommentsHandler>()?,
            get_tree_defaults_handler: locator.get::<GetTreeDefaultsHandler>()?,
            get_tree_handler: locator.get::<GetTreeHandler>()?,
            get_tree_history_handler: locator.get::<GetTreeHistoryHandler>()?,
            get_tree_stats_handler: locator.get::<GetTreeStatsHandler>()?,
            get_trees_handler: locator.get::<GetTreesHandler>()?,
            get_updated_trees_handler: locator.get::<GetUpdatedTreesHandler>()?,
            like_tree_handler: locator.get::<LikeTreeHandler>()?,
            login_google_v3_handler: locator.get::<LoginGoogleV3Handler>()?,
            login_osm_handler: locator.get::<LoginOsmHandler>()?,
            move_tree_handler: locator.get::<MoveTreeHandler>()?,
            replace_tree_handler: locator.get::<ReplaceTreeHandler>()?,
            search_species_handler: locator.get::<SearchSpeciesHandler>()?,
            search_streets_handler: locator.get::<SearchStreetsHandler>()?,
            suggest_species_handler: locator.get::<SuggestSpeciesHandler>()?,
            tree_page_handler: locator.get::<TreePageHandler>()?,
            unlike_tree_handler: locator.get::<UnlikeTreeHandler>()?,
            update_settings_handler: locator.get::<UpdateSettingsHandler>()?,
            update_tree_handler: locator.get::<UpdateTreeHandler>()?,
            update_tree_height_handler: locator.get::<UpdateTreeHeightHandler>()?,
            update_tree_location_handler: locator.get::<UpdateTreeLocationHandler>()?,
            update_tree_diameter_handler: locator.get::<UpdateTreeDiameterHandler>()?,
            update_tree_circumference_handler: locator.get::<UpdateTreeCircumferenceHandler>()?,
            update_tree_thumbnail_handler: locator.get::<UpdateTreeThumbnailHandler>()?,
            update_tree_state_handler: locator.get::<UpdateTreeStateHandler>()?,
            upload_handler: locator.get::<UploadHandler>()?,
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
}
