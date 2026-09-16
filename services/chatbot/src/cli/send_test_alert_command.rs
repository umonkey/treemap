use crate::domains::outbox::OutboxRepository;
use crate::infra::config::Config;
use std::sync::Arc;

const TEST_ALERT_TEXT: &str = "This is a test alert message.  An operator is testing message delivery.  You can safely delete this message.";

pub async fn send_test_alert_command() {
    let config = Config::from_env();

    let db = Arc::new(
        crate::infra::database::DatabaseClient::new(&config.database_path)
            .await
            .expect("Failed to initialize database"),
    );

    let outbox = OutboxRepository::new(db);

    if config.report_recipients.is_empty() {
        log::warn!("No report recipients configured; nothing to send.");
        return;
    }

    for recipient in &config.report_recipients {
        match outbox.enqueue(0, recipient, TEST_ALERT_TEXT, None).await {
            Ok(id) => log::info!("Enqueued test alert {} for recipient '{}'.", id, recipient),
            Err(e) => log::error!(
                "Failed to enqueue test alert for recipient '{}': {:?}",
                recipient,
                e
            ),
        }
    }
}
