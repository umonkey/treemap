/**
 * The tree service performs updates to the trees database.
 *
 * Reading is straight-forward, updates also log individual property changes
 * to be able to track changes over time.
 */
use log::debug;
use std::sync::Arc;

use crate::services::Database;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp};

pub struct Trees {
    db: Arc<dyn Database>,
}

impl Trees {
    pub async fn new(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn update_tree(&self, req: UpdateTreeRequest) -> Result<TreeRecord> {
        let now = get_timestamp();

        let old = self.get_tree(req.id).await?;

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
        };

        debug!("Updating tree: {:?}", new);

        self.db.update_tree(&new).await?;

        Ok(new)
    }

    pub async fn get_tree(&self, id: u64) -> Result<TreeRecord> {
        debug!("Getting details for tree {}.", id);

        match self.db.get_tree(id).await? {
            Some(tree) => Ok(tree),

            None => Err(Error::TreeNotFound),
        }
    }
}
