use crate::domains::outbox::OutboxRepository;
use crate::infra::config::Config;
use crate::services::outbox_dispatcher::OutboxDispatcher;
use std::sync::Arc;

pub async fn dispatch_outbox_command() {
    let config = Config::from_env();

    let db = Arc::new(
        crate::infra::database::DatabaseClient::new(&config.database_path)
            .await
            .expect("Failed to initialize database"),
    );

    let outbox = Arc::new(OutboxRepository::new(Arc::clone(&db)));

    log::info!("Starting Outbox Dispatcher daemon...");

    let dispatcher = OutboxDispatcher::new(config.bot_token, outbox);
    dispatcher.run().await;
}
