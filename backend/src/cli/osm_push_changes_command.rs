use crate::handlers::*;
use crate::services::*;

pub async fn osm_push_changes_command() {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.contains(&"--dry-run".to_string());

    let locator = Locator::new();

    let handler = locator
        .get::<OsmPushChangesHandler>()
        .expect("Error creating the handler.");

    handler
        .handle(dry_run)
        .await
        .expect("Error sending updates.");
}
