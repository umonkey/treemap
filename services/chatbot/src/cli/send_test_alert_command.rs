use crate::infra::config::Config;
use crate::infra::telegram::{parse_recipient, TelegramClient};

const TEST_ALERT_TEXT: &str = "This is a test alert message.  An operator is testing message delivery.  You can safely delete this message.";

pub async fn send_test_alert_command() {
    let config = Config::from_env();

    if config.report_recipients.is_empty() {
        log::warn!("No report recipients configured; nothing to send.");
        return;
    }

    let telegram = TelegramClient::new(config.bot_token);

    for recipient in &config.report_recipients {
        match parse_recipient(recipient) {
            Ok((chat_id, thread_id)) => {
                match telegram
                    .send_message(chat_id, thread_id, TEST_ALERT_TEXT, &[])
                    .await
                {
                    Ok(()) => log::info!("Sent test alert to recipient '{}'.", recipient),
                    Err(e) => log::error!(
                        "Failed to send test alert to recipient '{}': {:?}",
                        recipient,
                        e
                    ),
                }
            }
            Err(e) => log::error!("Invalid recipient '{}': {:?}", recipient, e),
        }
    }
}
