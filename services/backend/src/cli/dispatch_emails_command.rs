use crate::domain::email::EmailDispatcher;
use crate::services::AppState;

pub async fn dispatch_emails_command() {
    let state = match AppState::new().await {
        Ok(state) => state,
        Err(e) => {
            log::error!("Error initializing app state: {e}");
            std::process::exit(1);
        }
    };

    let dispatcher = match EmailDispatcher::new(&state.secrets, state.email.repo.clone()) {
        Ok(dispatcher) => dispatcher,
        Err(e) => {
            log::error!("Error creating email dispatcher: {e}");
            std::process::exit(1);
        }
    };

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
