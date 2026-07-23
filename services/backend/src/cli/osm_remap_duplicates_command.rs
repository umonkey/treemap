use crate::services::tree_merger::TreeMergerService;
use crate::services::*;

pub async fn osm_remap_duplicates_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.")
        .session()
        .await
        .expect("Error creating session state.");

    let merger = state
        .build::<TreeMergerService>()
        .expect("Error creating handler.");

    let remapped = merger
        .remap_osm_duplicates()
        .await
        .expect("Error remapping OSM duplicates.");

    state
        .database
        .commit()
        .await
        .expect("Error committing transaction.");

    for (main_id, dup_id) in remapped {
        println!(
            "Remapped OSM IDs for tree {} and duplicate {}.",
            main_id, dup_id
        );
    }
}
