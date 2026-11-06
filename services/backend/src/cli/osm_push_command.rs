use crate::domain::instance::InstanceService;
use crate::services::*;
use log::info;

pub async fn osm_push_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let instances = state
        .build::<InstanceService>()
        .expect("Error creating the instance service.")
        .list_for_osm_push()
        .await
        .expect("Error listing instances for OSM push.");

    if instances.is_empty() {
        info!("No instances have OSM push enabled.");
        return;
    }

    let state = state.session().await.expect("Error starting session.");

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
