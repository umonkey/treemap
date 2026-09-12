use crate::domain::panorama::PanoramaService;
use crate::services::{AppState, ContextExt};

pub async fn refresh_panorama_stats_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");
    let service = state
        .build::<PanoramaService>()
        .expect("Error creating panorama service.");
    let panoramas = service
        .get_all_panoramas()
        .await
        .expect("Error loading panoramas.");
    let count = panoramas.len();

    for panorama in panoramas {
        service
            .schedule_stats_refresh(panorama.id)
            .await
            .expect("Error scheduling refresh.");
    }

    println!("Scheduled stats refresh for {count} panoramas.");
}
