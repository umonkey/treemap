//! This module pushes local changes to OpenStreetMap.
//!
//! There are two types of changes:
//! (1) New trees found during surveys that don't have an OSM ID yet.
//! (2) Updates to existing trees that already have an OSM ID.

use crate::domain::osm::{OsmTreeRecord, OsmTreeRepository};
use crate::domain::tree::{Tree, TreeRepository};
use crate::infra::osm::{OsmClient, OsmElement};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use log::{debug, info, warn};
use std::sync::Arc;

// Don't push trees younger than 60 minutes, let users finish their surveys.
const MIN_AGE: u64 = 3600;

pub struct OsmWriterService {
    osm: Arc<OsmClient>,
    osm_trees: Arc<OsmTreeRepository>,
    trees: Arc<TreeRepository>,
    user_id: u64,
    changeset_size: usize,
}

impl OsmWriterService {
    /// Push new trees to OSM.
    pub async fn push_new_trees(&self) -> Result<()> {
        let trees = self.get_new_trees().await?;

        if trees.is_empty() {
            info!("No new trees to push to OSM.");
            return Ok(());
        }

        let changeset = self
            .osm
            .create_changeset("Adding new trees found during surveys.")
            .await?;

        for tree in trees {
            let osm_id = self.osm.create_tree(changeset, &tree).await?;

            self.trees
                .update_osm_id(tree.id, osm_id, self.user_id)
                .await?;

            self.osm_trees
                .add(&OsmTreeRecord {
                    id: osm_id,
                    lat: tree.lat,
                    lon: tree.lon,
                    genus: None,
                    species: Some(tree.species),
                    height: tree.height,
                    circumference: tree.circumference,
                    diameter_crown: tree.diameter,
                    ..Default::default()
                })
                .await?;
        }

        self.osm.close_changeset(changeset).await?;

        Ok(())
    }

    /// Push updates to existing trees on OSM.
    pub async fn push_updates(&self, dry_run: bool) -> Result<()> {
        let trees = self.get_changed_trees().await?;

        if trees.is_empty() {
            info!("No changes to push to OSM.");
            return Ok(());
        }

        let total = trees.len();
        let trees = trees
            .into_iter()
            .take(self.changeset_size)
            .collect::<Vec<_>>();

        debug!(
            "Have {} node updates for OSM, sending {}.",
            total,
            trees.len()
        );

        let changeset = self.get_changeset_id(dry_run).await?;

        for tree in trees {
            let node = match self.osm.get_node(tree.id).await? {
                Some(value) => value,

                None => {
                    warn!("OSM node {} not found, skipping update.", tree.id);
                    continue;
                }
            };

            let node_with_changes = self.merge_changes(node.clone(), &tree);

            if !dry_run {
                // Send updates to OSM.
                self.osm.update_tree(changeset, &node_with_changes).await?;

                // Update our existing record to avoid duplicate updates.
                self.osm_trees.update(&tree).await?;
            } else {
                debug!("Source node: {node:?}");
                debug!("Updated node: {node_with_changes:?}");
            }
        }

        if !dry_run {
            self.osm.close_changeset(changeset).await?;
        }

        Ok(())
    }

    async fn get_new_trees(&self) -> Result<Vec<Tree>> {
        let mut res = Vec::new();
        let max_ts = get_timestamp() - MIN_AGE;

        for tree in self.trees.all().await? {
            if tree.osm_id.is_some() || !tree.is_existing() {
                continue;
            }

            if tree.added_at > max_ts {
                continue;
            }

            res.push(tree);

            if res.len() == self.changeset_size {
                break;
            }
        }

        Ok(res)
    }

    async fn get_changeset_id(&self, dry_run: bool) -> Result<u64> {
        if dry_run {
            return Ok(0);
        }

        self.osm.create_changeset("Updating tree attributes.").await
    }

    async fn get_changed_trees(&self) -> Result<Vec<OsmTreeRecord>> {
        let mut res = Vec::new();

        for tree in self.trees.all().await? {
            if tree.osm_id.is_some() {
                let tree_record: OsmTreeRecord = (&tree).into();

                if self.has_changes(&tree_record).await? {
                    res.push(tree_record);
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

impl Injectable for OsmWriterService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();

        Ok(Self {
            osm: Arc::new(ctx.build::<OsmClient>()?),
            osm_trees: Arc::new(ctx.build::<OsmTreeRepository>()?),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            user_id: config.bot_user_id,
            changeset_size: config.osm_changeset_size as usize,
        })
    }
}
