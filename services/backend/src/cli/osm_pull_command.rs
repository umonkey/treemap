use crate::domain::instance::{Instance, InstanceService};
use crate::services::*;
use crate::types::Result;
use crate::utils::build_overpass_query;
use log::{error, info, warn};

// Pull OSM data for all instances with pull enabled.
//
// This command must never panic.  Errors are logged and reported to Sentry,
// tagged with the failed instance, so a single bad instance does not stop
// the others or abort the cron job.
pub async fn osm_pull_command() {
    let state = match AppState::new().await {
        Ok(state) => state,

        Err(e) => {
            report_error(&e, "init", None);
            return;
        }
    };

    let instances = match state.build::<InstanceService>() {
        Ok(service) => match service.list_for_osm_pull().await {
            Ok(instances) => instances,

            Err(e) => {
                report_error(&e, "list-instances", None);
                return;
            }
        },

        Err(e) => {
            report_error(&e, "list-instances", None);
            return;
        }
    };

    if instances.is_empty() {
        info!("No instances have OSM pull enabled.");
        return;
    }

    for instance in instances {
        if let Err(e) = pull_instance(&state, &instance).await {
            report_error(&e, "pull", Some(&instance));
        }
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

// Log an error and send it to Sentry, optionally tagged with the instance.
fn report_error(e: &crate::types::Error, stage: &str, instance: Option<&Instance>) {
    match instance {
        Some(instance) => {
            error!(
                "OSM pull failed at stage {stage} for {}: {e}",
                instance.domain
            );
        }

        None => {
            error!("OSM pull failed at stage {stage}: {e}");
        }
    }

    sentry::with_scope(
        |scope| {
            scope.set_tag("command", "osm-pull");
            scope.set_tag("stage", stage);

            if let Some(instance) = instance {
                scope.set_tag("instance", &instance.domain);
                scope.set_extra("instance_id", instance.id.into());
            }
        },
        || {
            sentry::capture_message(&e.to_string(), sentry::Level::Error);
        },
    );
}
