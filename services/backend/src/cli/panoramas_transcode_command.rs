use crate::domain::panorama::PanoramaService;
use crate::services::*;

pub async fn panoramas_transcode_command() {
    let value = match std::env::args().nth(2) {
        Some(value) => value,

        None => {
            println!("Usage: treemap panoramas-transcode panorama_id");
            return;
        }
    };

    let panorama_id: u64 = match value.parse() {
        Ok(panorama_id) => panorama_id,

        Err(_) => {
            println!("Error: panorama_id must be a number.");
            return;
        }
    };

    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let service = state
        .build::<PanoramaService>()
        .expect("Error creating panorama service.");

    service
        .transcode_panorama(panorama_id)
        .await
        .expect("Error transcoding panorama.");
}
