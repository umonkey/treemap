use crate::domains::alert::AlertRepository;
use crate::domains::alert_photo::AlertPhotoRepository;
use crate::domains::outbox::OutboxRepository;
use std::sync::Arc;
use std::time::Duration;

pub struct AlertDispatcher {
    alerts: Arc<AlertRepository>,
    photos: Arc<AlertPhotoRepository>,
    outbox: Arc<OutboxRepository>,
    recipients: Vec<i64>,
    website_url: String,
    files_base_url: String,
    poll_interval: Duration,
}

impl AlertDispatcher {
    pub fn new(
        _token: String,
        alerts: Arc<AlertRepository>,
        photos: Arc<AlertPhotoRepository>,
        outbox: Arc<OutboxRepository>,
        recipients: Vec<i64>,
        website_url: String,
        files_base_url: String,
    ) -> Self {
        Self {
            alerts,
            photos,
            outbox,
            recipients,
            website_url,
            files_base_url,
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
            let photos = self
                .photos
                .get_photos_by_alert_id(alert.id)
                .await
                .unwrap_or_default();
            let attachment_urls: Vec<String> = photos
                .into_iter()
                .map(|p| format!("{}{}", self.files_base_url, p))
                .collect();
            let attachments_opt = if attachment_urls.is_empty() {
                None
            } else {
                Some(attachment_urls.as_slice())
            };

            let description = alert
                .description
                .as_deref()
                .unwrap_or("Description was not provided.");

            let link = format!("{}/alert/{}/preview", self.website_url, alert.id);
            let trailer = format!("\n\n{}\n\n#alerts", link);
            // Ensure total length <= 1024, truncating description if needed
            let max_desc_len = 1024 - trailer.chars().count();
            let desc_chars: Vec<char> = description.chars().collect();
            let truncated_desc = if desc_chars.len() > max_desc_len {
                let sub: String = desc_chars
                    .into_iter()
                    .take(max_desc_len.saturating_sub(3))
                    .collect();
                format!("{}...", sub)
            } else {
                description.to_string()
            };
            let text = format!("{}\n\n{}\n\n#alerts", truncated_desc, link);

            log::info!(
                "Enqueuing report alert {} to outbox for recipients...",
                alert.id
            );

            for &recipient_id in &self.recipients {
                if let Err(e) = self
                    .outbox
                    .enqueue(alert.id, recipient_id, &text, attachments_opt)
                    .await
                {
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
