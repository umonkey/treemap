use crate::services::OsmWriterService;
use crate::services::*;

pub async fn osm_push_delete_command() {
    let dry_run = std::env::var("DRY").unwrap_or_default() == "yes";

    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let limit: u64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let state = state.session().await.expect("Error starting session.");

    let writer = state
        .build::<OsmWriterService>()
        .expect("Error creating OSM writer.");

    let deleted = writer
        .push_deletions(limit, dry_run)
        .await
        .expect("Error pushing deletions to OSM.");

    if !dry_run {
        state
            .database
            .commit()
            .await
            .expect("Error committing transaction.");
    }

    if dry_run {
        println!("Dry run complete. {} nodes would be deleted.", deleted);
    } else {
        println!("Successfully deleted {} nodes from OSM.", deleted);
    }
}
