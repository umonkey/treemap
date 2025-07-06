//! Push updates to existing trees on OSM.
//!
//! When we know that a tree exists in OSM, we have the osm_id,
//! and we can see that the tree has been updated, we push them.
//!
//! The algorithm is the following.
//!
//! (1) Get all trees from the database which have an osm_id.
//! (2) Convert them to OsmTreeRecord.
//! (3) See if there is a corresponding record in table osm_trees.
//! (4) See if that differs from the current tree.
//! (5) If it does, push the changes to OSM.

use crate::common::database::repositories::*;
use crate::config::Config;
use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::{debug, info, warn};
use std::sync::Arc;

pub struct OsmPushChangesHandler {
    osm: Arc<OsmClient>,
    osm_trees: Arc<OsmTreeRepository>,
    trees: Arc<TreeRepository>,
    changeset_size: u64,
    dry_run: bool,
}

impl OsmPushChangesHandler {
    pub async fn handle(&self) -> Result<()> {
        self.push_updates().await?;
        Ok(())
    }

    pub async fn push_updates(&self) -> Result<()> {
        let trees = self.get_changed_trees().await?;

        if trees.is_empty() {
            info!("No changes to push to OSM.");
            return Ok(());
        }

        let total = trees.len();
        let trees = trees
            .into_iter()
            .take(self.changeset_size as usize)
            .collect::<Vec<_>>();

        debug!(
            "Have {} node updates for OSM, sending {}.",
            total,
            trees.len()
        );

        let changeset = self.get_changeset_id().await?;

        for tree in trees {
            let node = match self.osm.get_node(tree.id).await? {
                Some(value) => value,

                None => {
                    warn!("OSM node {} not found, skipping update.", tree.id);
                    continue;
                }
            };

            let node_with_changes = self.merge_changes(node.clone(), &tree);

            if !self.dry_run {
                // Send updates to OSM.
                self.osm.update_tree(changeset, &node_with_changes).await?;

                // Update our existing record to avoid duplicate updates.
                self.osm_trees.update(&tree).await?;
            } else {
                debug!("Source node: {:?}", node);
                debug!("Updated node: {:?}", node_with_changes);
            }
        }

        if !self.dry_run {
            self.osm.close_changeset(changeset).await?;
        }

        Ok(())
    }

    async fn get_changeset_id(&self) -> Result<u64> {
        if self.dry_run {
            return Ok(0);
        }

        self.osm.create_changeset("Updating tree attributes.").await
    }

    async fn get_changed_trees(&self) -> Result<Vec<OsmTreeRecord>> {
        let mut res = Vec::new();

        for tree in self.trees.all().await? {
            if tree.osm_id.is_some() {
                let tree: OsmTreeRecord = (&tree).into();

                if self.has_changes(&tree).await? {
                    res.push(tree);
                }
            }
        }

        Ok(res)
    }

    async fn has_changes(&self, new: &OsmTreeRecord) -> Result<bool> {
        let osm_id = new.id;

        let old = match self.osm_trees.get(osm_id).await? {
            Some(value) => value,
            None => return Ok(false),
        };

        if old.lat != new.lat {
            debug!(
                "OSM node {} changed: lat: {} -> {}",
                osm_id, old.lat, new.lat
            );
            return Ok(true);
        }

        if old.lon != new.lon {
            debug!(
                "OSM node {} changed: lon: {} -> {}",
                osm_id, old.lon, new.lon
            );
            return Ok(true);
        }

        if old.species != new.species {
            debug!(
                "OSM node {} changed: species: {:?} -> {:?}",
                osm_id, old.species, new.species
            );
            return Ok(true);
        }

        if old.height.unwrap_or(0.0) != new.height.unwrap_or(0.0) {
            debug!(
                "OSM node {} changed: height: {:?} -> {:?}",
                osm_id, old.height, new.height
            );
            return Ok(true);
        }

        if old.circumference.unwrap_or(0.0) != new.circumference.unwrap_or(0.0) {
            debug!(
                "OSM node {} changed: circumference: {:?} -> {:?}",
                osm_id, old.circumference, new.circumference
            );
            return Ok(true);
        }

        if old.diameter_crown.unwrap_or(0.0) != new.diameter_crown.unwrap_or(0.0) {
            debug!(
                "OSM node {} changed: diameter_crown: {:?} -> {:?}",
                osm_id, old.diameter_crown, new.diameter_crown
            );
            return Ok(true);
        }

        Ok(false)
    }

    fn merge_changes(&self, node: OsmElement, tree: &OsmTreeRecord) -> OsmElement {
        let mut node = node;

        node.lat = tree.lat;
        node.lon = tree.lon;

        node.tags.insert("natural".to_string(), "tree".to_string());

        if let Some(value) = &tree.genus {
            node.tags.insert("genus".to_string(), value.to_string());
        }

        if let Some(value) = &tree.species {
            node.tags.insert("species".to_string(), value.to_string());
        }

        if let Some(value) = tree.height {
            if value > 0.0 {
                node.tags.insert("height".to_string(), value.to_string());
            }
        }

        if let Some(value) = tree.circumference {
            if value > 0.0 {
                node.tags
                    .insert("circumference".to_string(), value.to_string());
            }
        }

        if let Some(value) = tree.diameter_crown {
            if value > 0.0 {
                node.tags
                    .insert("diameter_crown".to_string(), value.to_string());
            }
        }

        if let Some(value) = &tree.image {
            node.tags.insert("image".to_string(), value.to_string());
        }

        node
    }
}

impl Locatable for OsmPushChangesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        Ok(Self {
            osm: locator.get::<OsmClient>()?,
            osm_trees: locator.get::<OsmTreeRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            changeset_size: config.osm_changeset_size,
            dry_run: get_dry_run()?,
        })
    }
}
