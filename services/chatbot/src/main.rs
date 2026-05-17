mod domains;
mod infra;
mod services;

use crate::domains::photo::PhotoRepository;
use crate::domains::report::ReportRepository;
use crate::domains::tree::TreeRepository;
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

    let reports = Arc::new(ReportRepository::new(Arc::clone(&db)));
    let photos = Arc::new(PhotoRepository::new(Arc::clone(&db)));
    let trees = Arc::new(TreeRepository::new(Arc::clone(&db)));

    services::chatbot::run(config.bot_token, i18n, reports, photos, trees).await;
}
