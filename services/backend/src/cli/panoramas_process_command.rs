use crate::domain::panorama::PanoramaService;
use crate::services::*;

pub async fn panoramas_process_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let service = state
        .build::<PanoramaService>()
        .expect("Error creating panorama service.");

    service
        .process_draft_panoramas()
        .await
        .expect("Error processing draft panoramas.");
}
