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

        match self.outbox.recover_stale_processing().await {
            Ok(n) if n > 0 => log::warn!("Recovered {} stale processing outbox message(s).", n),
            Ok(_) => {}
            Err(e) => log::error!(
                "Failed to recover stale processing outbox messages: {:?}",
                e
            ),
        }

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
            let telegram = Arc::clone(&self.telegram);
            let chat_id = msg.chat_id;
            let topic_id = msg.topic_id;
            let text = msg.text.clone();

            let send_res = match tokio::spawn(async move {
                telegram.send_message(chat_id, topic_id, &text, &urls).await
            })
            .await
            {
                Ok(res) => res,
                Err(join_err) => Err(anyhow::anyhow!("telegram send task failed: {join_err}")),
            };

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
