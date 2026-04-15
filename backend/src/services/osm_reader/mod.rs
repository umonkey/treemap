//! This module reads data from OSM and pulls changes to our local trees.
//!
//! The process has two steps:
//!
//! (1) Create a mirror of the current state of trees in OSM.  Use Overpass
//! to find all trees within configured boundaries, store their data in the
//! osm_trees table, which serves as a cache.  Deleted nodes are marked as
//! invisible.  The data includes node version and update timestamp to use
//! in conflict resolution.
//!
//! (2) Use the osm_trees table data to add new trees to our database, or
//! update existing trees that were changed on the OSM side.
//!
//! See details in docs/OSM-Integration.md

use crate::domain::osm::{OsmTreeRecord, OsmTreeRepository};
use crate::domain::tree::Tree;
use crate::domain::tree::TreeRepository;
use crate::infra::overpass::OverpassClient;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;

const DEFAULT_STATE: &str = "healthy";

pub struct OsmReaderService {
    trees: Arc<TreeRepository>,
    overpass_client: Arc<OverpassClient>,
    user_id: u64,
    osm_trees: Arc<OsmTreeRepository>,
}

impl OsmReaderService {
    // Synchronize the local mirror of OSM data.
    //
    // Uses the Overpass client to pull all tree nodes and put then in the osm_trees
    // table for use in the following data merging steps.
    //
    // Nodes that were not in the response, but existed previously, are considered
    // deleted and are marked as visible=0.
    pub async fn update_osm_cache(&self) -> Result<()> {
        info!("Running OSM reader service.");

        let sync_start = get_timestamp();

        let doc = self.overpass_client.query().await?;

        for mut node in doc.iter().cloned() {
            node.last_seen_at = Some(sync_start);
            node.visible = true;

            Self::update_cache_record(&self.osm_trees, &node).await?;
        }

        self.osm_trees.mark_invisible_before(sync_start).await?;

        info!("Found {} OSM nodes.", doc.len());

        Ok(())
    }

    // Iterate through OSM data and apply changes to local trees.
    pub async fn update_local_trees(&self) -> Result<()> {
        for node in self.osm_trees.all().await? {
            match self.trees.get_by_osm_id(node.id).await? {
                Some(tree) => self.update_tree(tree, node).await?,
                None => self.add_tree(node).await?,
            }
        }

        Ok(())
    }

    // Pull changes to an existing tree.
    async fn update_tree(&self, tree: Tree, node: OsmTreeRecord) -> Result<()> {
        if !node.visible {
            return self.delete_tree(tree).await;
        }

        // If OSM node is newer than local sync, update local tree.
        if node.version > tree.osm_version.unwrap_or(0) {
            let now = get_timestamp();
            let mut updated = tree.clone();
            updated.osm_id = Some(node.id);
            updated.osm_version = Some(node.version);
            updated.last_sync_at = Some(now);

            // Merge logic: if the tree was updated locally since the last sync,
            // we only update coordinates from OSM.  Otherwise we update all tags.
            let has_local_edits = tree.updated_at > tree.last_sync_at.unwrap_or(0);

            if !has_local_edits {
                updated.lat = node.lat;
                updated.lon = node.lon;
                updated.species = node.get_species();
                updated.height = node.height;
                updated.circumference = node.circumference;
                updated.diameter = node.diameter_crown;
            } else {
                updated.lat = node.lat;
                updated.lon = node.lon;
            }

            self.trees.update(&updated, self.user_id).await?;

            info!(
                "Tree {} updated from OSM node {} (version {} > {}).",
                tree.id,
                node.id,
                node.version,
                tree.osm_version.unwrap_or(0)
            );
        }

        Ok(())
    }

    // Add a new tree, which is in the OSM data, but not on our side.
    async fn add_tree(&self, node: OsmTreeRecord) -> Result<()> {
        // This node was deleted in OSM, no need to add on our side.
        if !node.visible {
            return Ok(());
        }

        let now = get_timestamp();

        let tree = Tree {
            id: get_unique_id()?,
            osm_id: Some(node.id),
            lat: node.lat,
            lon: node.lon,
            species: node.get_species(),
            height: node.height,
            circumference: node.circumference,
            diameter: node.diameter_crown,
            state: DEFAULT_STATE.to_string(),
            added_at: now,
            updated_at: now,
            updated_by: self.user_id,
            added_by: self.user_id,
            osm_version: Some(node.version),
            last_sync_at: Some(now),
            ..Default::default()
        };

        self.trees.add(&tree).await?;

        info!("Tree {} added from OSM node {}.", tree.id, node.id);

        Ok(())
    }

    // Delete a tree that was removed from OSM.
    async fn delete_tree(&self, tree: Tree) -> Result<()> {
        if tree.state != "gone" {
            self.trees.mark_gone(tree.id, self.user_id).await?;

            info!(
                "Tree {} marked as gone because OSM node {:?} was deleted.",
                tree.id, tree.osm_id
            );
        }

        Ok(())
    }

    // Update a single node in the OSM cache.
    async fn update_cache_record(repo: &OsmTreeRepository, node: &OsmTreeRecord) -> Result<()> {
        let rows = repo.update(node).await?;

        if rows > 0 {
            debug!("OSM node {} updated in the database.", node.id);
        } else {
            repo.add(node).await?;
            debug!("OSM node {} added to the database.", node.id);
        }

        Ok(())
    }
}

impl Injectable for OsmReaderService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();

        Ok(Self {
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            overpass_client: Arc::new(ctx.build::<OverpassClient>()?),
            user_id: config.bot_user_id,
            osm_trees: Arc::new(ctx.build::<OsmTreeRepository>()?),
        })
    }
}
