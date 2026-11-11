use crate::domain::panorama::PanoramaDispatcher;
use crate::services::*;
use crate::types::Result;

pub async fn scan_panoramas_command() -> Result<()> {
    let state = AppState::new().await?;

    let dispatcher = state.build::<PanoramaDispatcher>()?;

    dispatcher.scan_panoramas().await?;

    Ok(())
}
