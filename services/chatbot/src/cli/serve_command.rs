use crate::domains::alert::AlertRepository;
use crate::domains::alert_photo::AlertPhotoRepository;
use crate::domains::tree::TreeRepository;
use crate::infra::config::Config;
use crate::services::i18n::I18n;
use std::sync::Arc;

pub async fn serve_command() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let db = Arc::new(crate::infra::database::DatabaseClient::new(&config.database_path).await?);

    let alerts = Arc::new(AlertRepository::new(Arc::clone(&db)));
    let photos = Arc::new(AlertPhotoRepository::new(Arc::clone(&db)));
    let i18n = Arc::new(I18n::new());
    let trees = Arc::new(TreeRepository::new(Arc::clone(&db)));
    let storage = Arc::new(crate::infra::s3::S3FileStorage::new(&config)?);

    crate::services::chatbot::run(config.bot_token, i18n, alerts, photos, trees, storage).await;

    Ok(())
}
