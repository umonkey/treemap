//! Push updates to existing trees on OSM.
//!
//! When we know that a tree exists in OSM, we have the osm_id,
//! and we can see that the tree has been updated, we push them.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::{debug, info, warn};
use std::sync::Arc;

const MAX_CHANGES: usize = 100;

pub struct OsmPushChangesHandler {
    osm: Arc<OsmClient>,
    osm_trees: Arc<OsmTreeRepository>,
    trees: Arc<TreeRepository>,
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
        let trees = trees.into_iter().take(MAX_CHANGES).collect::<Vec<_>>();

        debug!(
            "Have {} node updates for OSM, sending {}.",
            total,
            trees.len()
        );

        let changeset = self
            .osm
            .create_changeset("Updating tree attributes.")
            .await?;

        for tree in trees {
            let node = match self.osm.get_node(tree.osm_id.unwrap()).await? {
                Some(value) => value,

                None => {
                    warn!(
                        "OSM node {} not found, skipping update.",
                        tree.osm_id.unwrap()
                    );
                    continue;
                }
            };

            let node_with_changes = self.merge_changes(node, &tree);

            self.osm.update_tree(changeset, &node_with_changes).await?;

            // Update our existing record to avoid duplicate updates.
            let osm_record: OsmTreeRecord = (&tree).into();
            self.osm_trees.update(&osm_record).await?;
        }

        self.osm.close_changeset(changeset).await?;

        Ok(())
    }

    async fn get_changed_trees(&self) -> Result<Vec<TreeRecord>> {
        let mut res = Vec::new();

        for tree in self.trees.all().await? {
            if !self.shall_add(&tree).await? {
                continue;
            }

            res.push(tree);
        }

        Ok(res)
    }

    async fn shall_add(&self, tree: &TreeRecord) -> Result<bool> {
        let osm_id = match tree.osm_id {
            Some(value) => value,
            None => return Ok(false),
        };

        let old = match self.osm_trees.get(osm_id).await? {
            Some(value) => value,
            None => return Ok(false),
        };

        let new: OsmTreeRecord = tree.into();

        if self.ll_diff(old.lat, new.lat) {
            debug!(
                "OSM node {} changed: lat: {} -> {}",
                osm_id, old.lat, new.lat
            );
            return Ok(true);
        }

        if self.ll_diff(old.lon, new.lon) {
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

    // Tell if a coordinate difference is more than 1 cm.
    // @docs https://support.garmin.com/en-US/?faq=hRMBoCTy5a7HqVkxukhHd8
    fn ll_diff(&self, a: f64, b: f64) -> bool {
        (a - b).abs() > 0.0000001
    }

    fn merge_changes(&self, node: OsmElement, tree: &TreeRecord) -> OsmElement {
        let mut node = node;

        node.lat = tree.lat;
        node.lon = tree.lon;

        node.tags.insert("natural".to_string(), "tree".to_string());

        if let Some(value) = tree.get_genus() {
            node.tags.insert("genus".to_string(), value);
        }

        if let Some(value) = tree.get_full_species() {
            node.tags.insert("species".to_string(), value);
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

        if let Some(value) = tree.diameter {
            if value > 0.0 {
                node.tags
                    .insert("diameter_crown".to_string(), value.to_string());
            }
        }

        if tree.thumbnail_id.is_some() {
            let page = format!("https://yerevan.treemaps.app/tree/{}", tree.id);
            node.tags.insert("image".to_string(), page.to_string());
        }

        node
    }
}

impl Locatable for OsmPushChangesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            osm: locator.get::<OsmClient>()?,
            osm_trees: locator.get::<OsmTreeRepository>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
