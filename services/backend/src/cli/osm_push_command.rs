use crate::domain::instance::InstanceService;
use crate::services::*;
use crate::types::Result;
use log::info;

pub async fn osm_push_command() -> Result<()> {
    let state = AppState::new().await?;

    let instances = state
        .build::<InstanceService>()?
        .list_for_osm_push()
        .await?;

    if instances.is_empty() {
        info!("No instances have OSM push enabled.");
        return Ok(());
    }

    let state = state.session().await?;

    let service = state.build::<OsmWriterService>()?;

    service.push_new_trees().await?;

    state.database.commit().await?;

    Ok(())
}
