use crate::services::OsmWriterService;
use crate::services::*;

pub async fn osm_push_delete_command() {
    let dry_run = std::env::var("DRY").unwrap_or_default() == "yes";

    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let writer = state
        .build::<OsmWriterService>()
        .expect("Error creating OSM writer.");

    let deleted = writer
        .push_deletions(dry_run)
        .await
        .expect("Error pushing deletions to OSM.");

    if dry_run {
        println!("Dry run complete. {} nodes would be deleted.", deleted);
    } else {
        println!("Successfully deleted {} nodes from OSM.", deleted);
    }
}
