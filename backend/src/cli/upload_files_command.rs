use crate::services::migrate::MigrateService;
use crate::services::{AppState, ContextExt};

pub async fn upload_files_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let migrate = state
        .build::<MigrateService>()
        .expect("Error creating handler.");

    migrate.migrate_files().await.expect("Error copying files.");
}
