use crate::infra::config::Config;
use crate::infra::database::Database;
use crate::infra::queue::Queue;
use crate::infra::tokens::TokenService;
use crate::services::Locator;
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub trait Context {
    fn database(&self) -> Arc<Database>;
    fn config(&self) -> Arc<Config>;
    fn queue(&self) -> Arc<Queue>;
    fn locator(&self) -> &Locator;
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

impl Context for Locator {
    fn database(&self) -> Arc<Database> {
        self.get::<Database>().expect("Database not found")
    }

    fn config(&self) -> Arc<Config> {
        self.get::<Config>().expect("Config not found")
    }

    fn queue(&self) -> Arc<Queue> {
        self.get::<Queue>().expect("Queue not found")
    }

    fn locator(&self) -> &Locator {
        self
    }
}

pub struct AppState {
    pub locator: Arc<Locator>,
    pub database: Arc<Database>,
    pub config: Arc<Config>,
    pub queue: Arc<Queue>,
    pub tokens: Arc<TokenService>,
}

impl AppState {
    pub async fn new(locator: Arc<Locator>) -> Result<Self> {
        Ok(Self {
            database: locator.get::<Database>()?,
            config: locator.get::<Config>()?,
            queue: locator.get::<Queue>()?,
            tokens: locator.get::<TokenService>()?,
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

    fn locator(&self) -> &Locator {
        &self.locator
    }
}
