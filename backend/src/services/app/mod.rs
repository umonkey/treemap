use crate::domain::comment::CommentService;
use crate::domain::health::*;
use crate::domain::heatmap::HeatmapService;
use crate::domain::like::LikeService;
use crate::domain::login::LoginService;
use crate::domain::observation::ObservationService;
use crate::domain::prop::PropService;
use crate::domain::settings::SettingsService;
use crate::domain::species::SpeciesService;
use crate::domain::stats::StatsService;
use crate::domain::street::StreetService;
use crate::domain::training::TrainingService;
use crate::domain::tree::TreeService;
use crate::domain::tree_image::TreeImageService;
use crate::domain::upload::UploadService;
use crate::domain::user::UserService;
use crate::infra::config::Config;
use crate::infra::database::Database;
use crate::infra::queue::Queue;
use crate::infra::tokens::TokenService;
use crate::services::comment_loader::CommentLoader;
use crate::services::like_loader::LikeLoader;
use crate::services::meta::MetaService;
use crate::services::prop_loader::PropLoader;
use crate::services::tree_loader::TreeLoader;
use crate::services::Locator;
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub trait Context {
    fn database(&self) -> Arc<Database>;
    fn config(&self) -> Arc<Config>;
    fn queue(&self) -> Arc<Queue>;
    fn locator(&self) -> Arc<Locator>;
}

pub trait Injectable {
    fn inject(ctx: &dyn Context) -> Result<Self>
    where
        Self: Sized;
}

impl dyn Context + '_ {
    pub fn build<T: Injectable>(&self) -> Result<T> {
        T::inject(self)
    }
}

pub trait ContextExt: Context {
    fn build<T: Injectable>(&self) -> Result<T>
    where
        Self: Sized,
    {
        T::inject(self)
    }
}

impl<T: Context> ContextExt for T {}

pub struct AppState {
    pub locator: Arc<Locator>,
    pub database: Arc<Database>,
    pub config: Arc<Config>,
    pub queue: Arc<Queue>,
    pub tree_images: Arc<TreeImageService>,
    tokens: Arc<TokenService>,
    pub users: Arc<UserService>,
    pub training: Arc<TrainingService>,
    pub trees: Arc<TreeService>,
    pub health: Arc<HealthService>,
    pub heatmap: Arc<HeatmapService>,
    pub likes: Arc<LikeService>,
    pub observations: Arc<ObservationService>,
    pub comments: Arc<CommentService>,
    pub login: Arc<LoginService>,
    pub species: Arc<SpeciesService>,
    pub settings: Arc<SettingsService>,
    pub uploads: Arc<UploadService>,
    pub stats: Arc<StatsService>,
    pub streets: Arc<StreetService>,
    pub tree_loader: Arc<TreeLoader>,
    pub comment_loader: Arc<CommentLoader>,
    pub like_loader: Arc<LikeLoader>,
    pub meta: Arc<MetaService>,
    pub props: Arc<PropService>,
    pub prop_loader: Arc<PropLoader>,
}

impl AppState {
    pub async fn new(locator: Arc<Locator>) -> Result<Self> {
        Ok(Self {
            database: locator.get::<Database>()?,
            config: locator.get::<Config>()?,
            queue: locator.get::<Queue>()?,
            tree_images: locator.get::<TreeImageService>()?,
            tokens: locator.get::<TokenService>()?,
            users: locator.get::<UserService>()?,
            training: locator.get::<TrainingService>()?,
            trees: locator.get::<TreeService>()?,
            health: locator.get::<HealthService>()?,
            heatmap: locator.get::<HeatmapService>()?,
            comments: locator.get::<CommentService>()?,
            observations: locator.get::<ObservationService>()?,
            login: locator.get::<LoginService>()?,
            species: locator.get::<SpeciesService>()?,
            settings: locator.get::<SettingsService>()?,
            uploads: locator.get::<UploadService>()?,
            stats: locator.get::<StatsService>()?,
            streets: locator.get::<StreetService>()?,
            likes: locator.get::<LikeService>()?,
            tree_loader: locator.get::<TreeLoader>()?,
            comment_loader: locator.get::<CommentLoader>()?,
            like_loader: locator.get::<LikeLoader>()?,
            meta: locator.get::<MetaService>()?,
            props: locator.get::<PropService>()?,
            prop_loader: locator.get::<PropLoader>()?,
            locator,
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

impl Context for AppState {
    fn database(&self) -> Arc<Database> {
        self.database.clone()
    }

    fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    fn queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }

    fn locator(&self) -> Arc<Locator> {
        self.locator.clone()
    }
}
