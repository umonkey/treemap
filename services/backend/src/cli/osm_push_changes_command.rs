use crate::services::*;
use crate::types::Result;

pub async fn osm_push_changes_command() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.contains(&"--dry-run".to_string());

    let state = AppState::new().await?.session().await?;

    let service = state.build::<OsmWriterService>()?;

    service.push_updates(dry_run).await?;

    if !dry_run {
        state.database.commit().await?;
    }

    Ok(())
}
