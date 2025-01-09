use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp};
use log::info;
use std::sync::Arc;

pub struct UpdateTreeHandler {
    trees: Arc<TreeRepository>,
    queue: Arc<QueueService>,
}

impl UpdateTreeHandler {
    pub async fn handle(&self, req: UpdateTreeRequest) -> Result<TreeRecord> {
        let now = get_timestamp();

        let old = match self.trees.get(req.id).await? {
            Some(tree) => tree,
            None => return Err(Error::TreeNotFound),
        };

        let new = TreeRecord {
            id: req.id,
            osm_id: old.osm_id,
            lat: old.lat,
            lon: old.lon,
            species: req.species.unwrap_or(old.species),
            notes: match req.notes {
                Some(value) => Some(value),
                None => old.notes,
            },
            height: match req.height {
                Some(value) => Some(value),
                None => old.height,
            },
            circumference: match fix_circumference(req.circumference) {
                Some(value) => Some(value),
                None => old.circumference,
            },
            diameter: match req.diameter {
                Some(value) => Some(value),
                None => old.diameter,
            },
            state: match req.state {
                Some(value) => value,
                None => old.state,
            },
            added_at: old.added_at,
            updated_at: now,
            added_by: old.added_by,
            thumbnail_id: old.thumbnail_id,
            year: match req.year {
                Some(value) => Some(value),
                None => old.year,
            },
            address: match req.address {
                Some(value) => Some(value),
                None => old.address,
            },
        };

        info!("Updating tree: {:?}", new);

        self.trees.update(&new, req.user_id).await?;

        if new.address.is_none() {
            self.schedule_address_update(new.id).await?;
        }

        Ok(new)
    }

    async fn schedule_address_update(&self, tree_id: u64) -> Result<()> {
        let msg = UpdateTreeAddressMessage { id: tree_id };
        self.queue.push(&msg.encode()).await?;

        info!("Scheduled address update for tree {}", tree_id);

        Ok(())
    }
}

impl Locatable for UpdateTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            queue: locator.get::<QueueService>()?,
        })
    }
}
