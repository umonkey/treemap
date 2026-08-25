use crate::domains::alert::AlertRepository;
use crate::domains::outbox::OutboxRepository;
use std::sync::Arc;
use std::time::Duration;

pub struct AlertDispatcher {
    alerts: Arc<AlertRepository>,
    outbox: Arc<OutboxRepository>,
    recipients: Vec<i64>,
    website_url: String,
    poll_interval: Duration,
}

impl AlertDispatcher {
    pub fn new(
        _token: String,
        alerts: Arc<AlertRepository>,
        outbox: Arc<OutboxRepository>,
        recipients: Vec<i64>,
        website_url: String,
    ) -> Self {
        Self {
            alerts,
            outbox,
            recipients,
            website_url,
            poll_interval: Duration::from_secs(10),
        }
    }

    pub async fn run(self) {
        if self.recipients.is_empty() {
            log::warn!("AlertDispatcher running, but REPORT_RECIPIENTS is empty. No notifications will be sent.");
        } else {
            log::info!(
                "AlertDispatcher running with {} recipient(s) (poll interval: {:?}).",
                self.recipients.len(),
                self.poll_interval
            );
        }

        loop {
            if let Err(e) = self.dispatch_once().await {
                log::error!("Error in AlertDispatcher tick: {:?}", e);
            }

            tokio::time::sleep(self.poll_interval).await;
        }
    }

    async fn dispatch_once(&self) -> anyhow::Result<()> {
        if self.recipients.is_empty() {
            return Ok(());
        }

        let unreported = self.alerts.get_unreported().await?;
        if unreported.is_empty() {
            log::debug!("AlertDispatcher: no unreported alerts found.");
            return Ok(());
        }

        log::debug!(
            "AlertDispatcher: found {} unreported alert(s).",
            unreported.len()
        );

        for alert in unreported {
            let description = alert
                .description
                .as_deref()
                .unwrap_or("Description was not provided.");

            let text = format!(
                "New report available:\n\n{}/alert/{}/preview\n\n{}",
                self.website_url, alert.id, description
            );

            log::info!(
                "Enqueuing report alert {} to outbox for recipients...",
                alert.id
            );

            for &recipient_id in &self.recipients {
                if let Err(e) = self.outbox.enqueue(alert.id, recipient_id, &text).await {
                    log::error!(
                        "Failed to enqueue alert {} for recipient {}: {:?}",
                        alert.id,
                        recipient_id,
                        e
                    );
                }
            }

            self.alerts.mark_reported(alert.id).await?;
            log::info!("Alert {} marked as reported and enqueued.", alert.id);
        }

        Ok(())
    }
}
