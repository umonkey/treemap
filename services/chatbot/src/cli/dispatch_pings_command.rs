use crate::domains::alert::AlertRepository;
use crate::domains::outbox::OutboxRepository;
use crate::infra::config::Config;
use crate::services::i18n::I18n;
use crate::services::pinger::AlertPinger;
use std::sync::Arc;

pub async fn dispatch_pings_command() {
    let config = Config::from_env();

    let db = Arc::new(
        crate::infra::database::DatabaseClient::new(&config.database_path)
            .await
            .expect("Failed to initialize database"),
    );

    let alerts = Arc::new(AlertRepository::new(Arc::clone(&db)));
    let outbox = Arc::new(OutboxRepository::new(Arc::clone(&db)));
    let i18n = Arc::new(I18n::new());

    log::info!("Starting Alert Pinger daemon...");

    let pinger = AlertPinger::new(alerts, outbox, i18n);
    pinger.run().await;
}
