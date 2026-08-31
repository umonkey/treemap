use crate::domains::alert::AlertRepository;
use crate::domains::outbox::OutboxRepository;
use crate::services::i18n::I18n;
use std::sync::Arc;
use std::time::Duration;

pub struct AlertPinger {
    alerts: Arc<AlertRepository>,
    outbox: Arc<OutboxRepository>,
    i18n: Arc<I18n>,
    poll_interval: Duration,
}

impl AlertPinger {
    pub fn new(
        alerts: Arc<AlertRepository>,
        outbox: Arc<OutboxRepository>,
        i18n: Arc<I18n>,
    ) -> Self {
        Self {
            alerts,
            outbox,
            i18n,
            poll_interval: Duration::from_secs(10),
        }
    }

    pub async fn run(self) {
        log::info!("AlertPinger running daemon loop.");

        loop {
            if let Err(e) = self.ping_once().await {
                log::error!("Error in AlertPinger tick: {:?}", e);
            }

            tokio::time::sleep(self.poll_interval).await;
        }
    }

    async fn ping_once(&self) -> anyhow::Result<()> {
        let pending = self.alerts.get_pending_pings().await?;
        if pending.is_empty() {
            return Ok(());
        }

        for alert in pending {
            let raw_lang = alert.language_code.as_deref().unwrap_or("en");
            let lang = &raw_lang[..2.min(raw_lang.len())];
            let text = self.i18n.tr("alert-ping-reminder", lang, None);

            log::info!(
                "Enqueuing ping reminder for alert {} (chat {}) to outbox...",
                alert.id,
                alert.chat_id
            );

            if let Err(e) = self
                .outbox
                .enqueue(alert.id, alert.chat_id, &text, None)
                .await
            {
                log::error!(
                    "Failed to enqueue ping reminder for alert {}: {:?}",
                    alert.id,
                    e
                );
                continue;
            }

            if let Err(e) = self.alerts.disable_ping(alert.id).await {
                log::error!("Failed to disable ping for alert {}: {:?}", alert.id, e);
            }
        }

        Ok(())
    }
}
