use crate::services::*;

pub async fn osm_push_changes_command() {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.contains(&"--dry-run".to_string());

    let locator = Locator::new();

    let service = locator
        .build::<OsmWriterService>()
        .expect("Error creating the service.");

    service
        .push_updates(dry_run)
        .await
        .expect("Error sending updates.");
}
