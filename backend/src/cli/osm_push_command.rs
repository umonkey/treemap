use crate::services::*;

pub async fn osm_push_command() {
    let locator = Locator::new();

    let service = locator
        .build::<OsmWriterService>()
        .expect("Error creating the service.");

    service
        .push_new_trees()
        .await
        .expect("Error processing updates.");
}
