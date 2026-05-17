mod infra;
mod services;

use crate::infra::config::Config;
use crate::services::i18n::I18n;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::from_env();
    let i18n = Arc::new(I18n::new());
    let db = Arc::new(
        crate::infra::database::DatabaseClient::new(&config.database_path)
            .await
            .expect("Failed to initialize database"),
    );

    services::chatbot::run(config.bot_token, i18n, db).await;
}
