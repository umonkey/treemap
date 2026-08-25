use crate::domains::alert::AlertRepository;
use crate::domains::outbox::OutboxRepository;
use crate::infra::config::Config;
use crate::services::dispatcher::AlertDispatcher;
use std::sync::Arc;

pub async fn dispatch_alerts_command() {
    let config = Config::from_env();

    let db = Arc::new(
        crate::infra::database::DatabaseClient::new(&config.database_path)
            .await
            .expect("Failed to initialize database"),
    );

    let alerts = Arc::new(AlertRepository::new(Arc::clone(&db)));
    let outbox = Arc::new(OutboxRepository::new(Arc::clone(&db)));

    log::info!("Starting Alert Dispatcher daemon...");

    let dispatcher = AlertDispatcher::new(
        config.bot_token,
        alerts,
        outbox,
        config.report_recipients,
        config.website_url,
    );
    dispatcher.run().await;
}
