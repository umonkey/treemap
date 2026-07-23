use crate::infra::config::Config;
use crate::infra::database::Database;
use crate::infra::queue::Queue;
use crate::infra::secrets::Secrets;
use crate::infra::storage::{create_driver, BackupBucket, FileBucket};
use crate::infra::tokens::TokenService;
use crate::services::mcp::McpSessionManager;
use crate::types::*;
use actix_web::HttpRequest;
use std::sync::Arc;

pub trait Context {
    fn database(&self) -> Arc<Database>;
    fn config(&self) -> Arc<Config>;
    fn queue(&self) -> Arc<Queue>;
    fn secrets(&self) -> Arc<Secrets>;
    fn tokens(&self) -> Arc<TokenService>;
    fn storage(&self) -> Arc<FileBucket>;
    #[allow(dead_code)]
    fn backups(&self) -> Arc<BackupBucket>;
    #[allow(dead_code)]
    fn mcp(&self) -> Arc<McpSessionManager>;
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

mod injected;
pub use injected::Injected;

mod middleware;
pub use middleware::Transaction;

mod user_id;
pub use user_id::{OptionalUserId, UserId};

mod require_permission;
pub use require_permission::*;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
    pub queue: Arc<Queue>,
    pub secrets: Arc<Secrets>,
    pub tokens: Arc<TokenService>,
    pub storage: Arc<FileBucket>,
    pub backups: Arc<BackupBucket>,
    pub mcp: Arc<McpSessionManager>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let config = Arc::new(Config::new()?);
        let secrets = Arc::new(Secrets::new(config.clone())?);
        let database = Arc::new(Database::new(&config, &secrets).await?);
        let queue = Arc::new(Queue::new(&config, &secrets, &database)?);

        let jwt_secret = secrets.jwt_secret.clone().ok_or(Error::Config(
            "JWT_SECRET not set, cannot use tokens".to_string(),
        ))?;
        let tokens = Arc::new(TokenService::new(jwt_secret));

        let driver = create_driver(&config, &secrets)?;

        let files_bucket = config.files_bucket.clone().unwrap_or("files".to_string());
        let storage = Arc::new(FileBucket::new(driver.clone(), files_bucket));

        let backups = Arc::new(BackupBucket::new(driver.clone(), &config)?);

        let mcp = Arc::new(McpSessionManager::default());

        Ok(Self {
            database,
            config,
            queue,
            secrets,
            tokens,
            storage,
            backups,
            mcp,
        })
    }

    pub async fn session(&self) -> Result<Self> {
        let database = Arc::new(self.database.transact().await?);

        Ok(Self {
            database,
            config: self.config.clone(),
            queue: self.queue.clone(),
            secrets: self.secrets.clone(),
            tokens: self.tokens.clone(),
            storage: self.storage.clone(),
            backups: self.backups.clone(),
            mcp: self.mcp.clone(),
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

    fn secrets(&self) -> Arc<Secrets> {
        self.secrets.clone()
    }

    fn tokens(&self) -> Arc<TokenService> {
        self.tokens.clone()
    }

    fn storage(&self) -> Arc<FileBucket> {
        self.storage.clone()
    }

    fn backups(&self) -> Arc<BackupBucket> {
        self.backups.clone()
    }

    fn mcp(&self) -> Arc<McpSessionManager> {
        self.mcp.clone()
    }
}
