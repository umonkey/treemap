//! This class is responsible for adding new trees.
//!
//! Before adding a new tree record, it ensures that there isn't a visible tree
//! with the same coordinates.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::{Error, Result, TreeRecord};
use log::debug;
use std::sync::Arc;

const DISTANCE: f64 = 0.1; // 10 cm

pub struct TreeInjector {
    trees: Arc<TreeRepository>,
}

impl TreeInjector {
    /// Add a tree, if there isn't a visible tree within 10 cm.
    pub async fn add(&self, tree: &TreeRecord) -> Result<()> {
        if self.exists_with_coordinates(tree.lat, tree.lon).await? {
            return Err(Error::DuplicateTree);
        }

        self.trees.add(tree).await
    }

    async fn exists_with_coordinates(&self, lat: f64, lon: f64) -> Result<bool> {
        for tree in self.trees.get_close(lat, lon, DISTANCE).await? {
            if tree.state != "gone" {
                debug!(
                    "Tree {} already exists at coordinates ({}, {})",
                    tree.id, lat, lon
                );
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Locatable for TreeInjector {
    fn create(locator: &Locator) -> Result<Self> {
        let trees = locator.get::<TreeRepository>()?;
        Ok(Self { trees })
    }
}
