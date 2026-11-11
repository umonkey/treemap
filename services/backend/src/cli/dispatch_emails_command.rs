use crate::domain::email::EmailDispatcher;
use crate::services::AppState;
use crate::types::Result;

pub async fn dispatch_emails_command() -> Result<()> {
    let state = AppState::new().await?;

    let dispatcher = EmailDispatcher::new(&state.config, state.email.repo.clone())?;

    log::info!("Running email dispatcher worker.");

    loop {
        match dispatcher.process_pending().await {
            Ok(count) => {
                if count == 0 {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
            Err(e) => {
                log::error!("Error processing pending emails: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
