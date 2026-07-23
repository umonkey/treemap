use crate::services::*;

pub async fn osm_push_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.")
        .session()
        .await
        .expect("Error starting session.");

    let service = state
        .build::<OsmWriterService>()
        .expect("Error creating the service.");

    service
        .push_new_trees()
        .await
        .expect("Error processing updates.");

    state
        .database
        .commit()
        .await
        .expect("Error committing transaction.");
}
