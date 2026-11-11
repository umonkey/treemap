use crate::domain::panorama::PanoramaService;
use crate::services::{AppState, ContextExt};
use crate::types::Result;

pub async fn refresh_panorama_stats_command() -> Result<()> {
    let state = AppState::new().await?;
    let service = state.build::<PanoramaService>()?;
    let panoramas = service.get_all_panoramas().await?;
    let count = panoramas.len();

    for panorama in panoramas {
        service.schedule_stats_refresh(panorama.id).await?;
    }

    println!("Scheduled stats refresh for {count} panoramas.");

    Ok(())
}
