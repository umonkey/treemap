use crate::services::tree_merger::TreeMergerService;
use crate::services::*;
use crate::types::Result;

pub async fn osm_remap_duplicates_command() -> Result<()> {
    let state = AppState::new().await?.session().await?;

    let merger = state.build::<TreeMergerService>()?;

    let remapped = merger.remap_osm_duplicates().await?;

    state.database.commit().await?;

    for (main_id, dup_id) in remapped {
        println!(
            "Remapped OSM IDs for tree {} and duplicate {}.",
            main_id, dup_id
        );
    }

    Ok(())
}
