use crate::domains::outbox::OutboxRepository;
use crate::infra::telegram::TelegramClient;
use std::sync::Arc;
use std::time::Duration;

pub struct OutboxDispatcher {
    telegram: Arc<TelegramClient>,
    outbox: Arc<OutboxRepository>,
    poll_interval: Duration,
}

impl OutboxDispatcher {
    pub fn new(telegram: Arc<TelegramClient>, outbox: Arc<OutboxRepository>) -> Self {
        Self {
            telegram,
            outbox,
            poll_interval: Duration::from_secs(10),
        }
    }

    pub async fn run(self) {
        log::info!("OutboxDispatcher running daemon loop.");

        loop {
            if let Err(e) = self.dispatch_once().await {
                log::error!("Error in OutboxDispatcher tick: {:?}", e);
            }

            tokio::time::sleep(self.poll_interval).await;
        }
    }

    async fn dispatch_once(&self) -> anyhow::Result<()> {
        let messages = self.outbox.fetch_pending(20).await?;
        if messages.is_empty() {
            return Ok(());
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        for msg in messages {
            log::info!(
                "Sending outbox message {} to chat {}...",
                msg.id,
                msg.chat_id
            );

            let urls = msg.attachment_urls();
            let send_res = self
                .telegram
                .send_message(msg.chat_id, msg.topic_id, &msg.text, &urls)
                .await;

            match send_res {
                Ok(_) => {
                    if let Err(e) = self.outbox.mark_sent(msg.id).await {
                        log::error!("Failed to mark outbox message {} as sent: {:?}", msg.id, e);
                    } else {
                        log::info!("Outbox message {} sent successfully.", msg.id);
                    }
                }
                Err(e) => {
                    let err_msg = format!("{:?}", e);
                    log::error!("Failed to send outbox message {}: {}", msg.id, err_msg);

                    let next_attempts = msg.attempts + 1;
                    let delay = 10 * i64::pow(2, std::cmp::min(next_attempts, 6) as u32);
                    let next_retry_at = now + delay;

                    if let Err(mark_err) = self
                        .outbox
                        .mark_failed(msg.id, &err_msg, next_retry_at)
                        .await
                    {
                        log::error!(
                            "Failed to mark outbox message {} as failed: {:?}",
                            msg.id,
                            mark_err
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
