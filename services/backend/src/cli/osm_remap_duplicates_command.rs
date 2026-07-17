use crate::services::tree_merger::TreeMergerService;
use crate::services::*;

pub async fn osm_remap_duplicates_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let merger = state
        .build::<TreeMergerService>()
        .expect("Error creating handler.");

    let remapped = merger
        .remap_osm_duplicates()
        .await
        .expect("Error remapping OSM duplicates.");

    for (main_id, dup_id) in remapped {
        println!(
            "Remapped OSM IDs for tree {} and duplicate {}.",
            main_id, dup_id
        );
    }
}
