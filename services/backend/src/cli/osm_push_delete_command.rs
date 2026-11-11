use crate::services::OsmWriterService;
use crate::services::*;
use crate::types::Result;

pub async fn osm_push_delete_command() -> Result<()> {
    let dry_run = std::env::var("DRY").unwrap_or_default() == "yes";

    let state = AppState::new().await?;

    let limit: u64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let state = state.session().await?;

    let writer = state.build::<OsmWriterService>()?;

    let deleted = writer.push_deletions(limit, dry_run).await?;

    if !dry_run {
        state.database.commit().await?;
    }

    if dry_run {
        println!("Dry run complete. {} nodes would be deleted.", deleted);
    } else {
        println!("Successfully deleted {} nodes from OSM.", deleted);
    }

    Ok(())
}
