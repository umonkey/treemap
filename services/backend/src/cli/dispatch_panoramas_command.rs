use crate::domain::panorama::PanoramaDispatcher;
use crate::services::*;

pub async fn dispatch_panoramas_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let dispatcher = state
        .build::<PanoramaDispatcher>()
        .expect("Error creating panorama dispatcher.");

    dispatcher
        .process_draft_panoramas()
        .await
        .expect("Error processing draft panoramas.");
}
