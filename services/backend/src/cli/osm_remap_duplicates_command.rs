use crate::domain::tree::TreeService;
use crate::services::*;

pub async fn osm_remap_duplicates_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let trees = state
        .build::<TreeService>()
        .expect("Error creating handler.");

    let remapped = trees
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
