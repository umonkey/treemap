use crate::services::*;

pub async fn osm_push_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let service = state
        .build::<OsmWriterService>()
        .expect("Error creating the service.");

    service
        .push_new_trees()
        .await
        .expect("Error processing updates.");
}
