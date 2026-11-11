use crate::services::migrate::MigrateService;
use crate::services::{AppState, ContextExt};
use crate::types::Result;

pub async fn upload_files_command() -> Result<()> {
    let state = AppState::new().await?;

    let migrate = state.build::<MigrateService>()?;

    migrate.migrate_files().await?;

    Ok(())
}
