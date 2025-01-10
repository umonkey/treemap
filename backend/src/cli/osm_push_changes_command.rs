use crate::handlers::*;
use crate::services::*;

pub async fn osm_push_changes_command() {
    let locator = Locator::new();

    let handler = locator
        .get::<OsmPushChangesHandler>()
        .expect("Error creating the handler.");

    handler.handle().await.expect("Error sending updates.");
}
