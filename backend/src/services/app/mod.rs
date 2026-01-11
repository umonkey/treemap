use crate::domain::comment::CommentService;
use crate::domain::file::FileService;
use crate::domain::health::*;
use crate::domain::heatmap::HeatmapService;
use crate::domain::like::LikeService;
use crate::domain::login::LoginService;
use crate::domain::species::SpeciesService;
use crate::domain::stats::StatsService;
use crate::domain::street::StreetService;
use crate::domain::training::TrainingService;
use crate::domain::tree::TreeService;
use crate::domain::user::UserService;
use crate::handlers::*;
use crate::infra::tokens::TokenService;
use crate::services::comment_loader::CommentLoader;
use crate::services::like_loader::LikeLoader;
use crate::services::meta::MetaService;
use crate::services::tree_loader::TreeLoader;
use crate::services::Locator;
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub struct AppState {
    pub files: Arc<FileService>,
    tokens: Arc<TokenService>,
    pub users: Arc<UserService>,
    pub training: Arc<TrainingService>,
    pub trees: Arc<TreeService>,
    pub health: Arc<HealthService>,
    pub heatmap: Arc<HeatmapService>,
    pub likes: Arc<LikeService>,
    pub comments: Arc<CommentService>,
    pub login: Arc<LoginService>,
    pub species: Arc<SpeciesService>,
    pub update_settings_handler: Arc<UpdateSettingsHandler>,
    pub upload_handler: Arc<UploadHandler>,
    pub stats: Arc<StatsService>,
    pub streets: Arc<StreetService>,
    pub tree_loader: Arc<TreeLoader>,
    pub comment_loader: Arc<CommentLoader>,
    pub like_loader: Arc<LikeLoader>,
    pub meta: Arc<MetaService>,
}

impl AppState {
    pub async fn new(locator: Arc<Locator>) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileService>()?,
            tokens: locator.get::<TokenService>()?,
            users: locator.get::<UserService>()?,
            training: locator.get::<TrainingService>()?,
            trees: locator.get::<TreeService>()?,
            health: locator.get::<HealthService>()?,
            heatmap: locator.get::<HeatmapService>()?,
            comments: locator.get::<CommentService>()?,
            login: locator.get::<LoginService>()?,
            species: locator.get::<SpeciesService>()?,
            update_settings_handler: locator.get::<UpdateSettingsHandler>()?,
            upload_handler: locator.get::<UploadHandler>()?,
            stats: locator.get::<StatsService>()?,
            streets: locator.get::<StreetService>()?,
            likes: locator.get::<LikeService>()?,
            tree_loader: locator.get::<TreeLoader>()?,
            comment_loader: locator.get::<CommentLoader>()?,
            like_loader: locator.get::<LikeLoader>()?,
            meta: locator.get::<MetaService>()?,
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
