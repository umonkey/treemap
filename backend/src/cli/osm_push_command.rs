use crate::handlers::*;
use crate::services::*;

pub async fn osm_push_command() {
    let locator = Locator::new();

    let handler = locator
        .get::<OsmPushHandler>()
        .expect("Error creating the handler.");

    handler.handle().await.expect("Error processing updates.");
}
