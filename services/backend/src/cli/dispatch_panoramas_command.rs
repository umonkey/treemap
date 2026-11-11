use crate::domain::panorama::PanoramaDispatcher;
use crate::services::*;
use crate::types::Result;

pub async fn dispatch_panoramas_command() -> Result<()> {
    let state = AppState::new().await?;

    let dispatcher = state.build::<PanoramaDispatcher>()?;

    dispatcher.process_draft_panoramas().await?;

    Ok(())
}
