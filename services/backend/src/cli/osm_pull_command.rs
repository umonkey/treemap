use crate::domain::instance::{Instance, InstanceService};
use crate::services::*;
use crate::types::Result;
use crate::utils::build_overpass_query;
use log::{error, info, warn};

pub async fn osm_pull_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let instances = state
        .build::<InstanceService>()
        .expect("Error creating instance service.")
        .list_for_osm_pull()
        .await
        .expect("Error listing instances for OSM pull.");

    if instances.is_empty() {
        info!("No instances have OSM pull enabled.");
        return;
    }

    let mut failures = 0;

    for instance in instances {
        if let Err(e) = pull_instance(&state, &instance).await {
            error!("OSM pull failed for {}: {e}", instance.domain);
            failures += 1;
        }
    }

    if failures > 0 {
        panic!("OSM pull failed for {failures} instance(s).");
    }
}

async fn pull_instance(state: &AppState, instance: &Instance) -> Result<()> {
    let Some(bbox) = instance.bbox() else {
        warn!(
            "Skipping OSM pull for {}: no bounding box configured.",
            instance.domain
        );
        return Ok(());
    };

    let user_id = instance
        .osm_bot_id
        .filter(|id| *id > 0)
        .unwrap_or(state.config.bot_user_id);

    if user_id == 0 {
        warn!(
            "Skipping OSM pull for {}: no bot user id configured.",
            instance.domain
        );
        return Ok(());
    }

    let query = build_overpass_query(bbox);

    let session = state.session().await?;

    let service = session.build::<OsmReaderService>()?;

    // Pull new tree nodes, put them in the osm_trees table.
    service.update_osm_cache(&query, bbox).await?;

    // Match new OSM nodes to local trees using coordinates.
    service.update_local_trees(bbox, user_id).await?;

    session.database.commit().await?;

    Ok(())
}
