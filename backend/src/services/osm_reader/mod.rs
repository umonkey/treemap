//! This module reads data from OSM and puts it into the database.
//!
//! The workflow is the following:
//!
//! 1. Query OSM for data.
//! 2. Parse the data.
//! 3. Filter out the data that is not trees.
//! 4. Add the data to the database.
//!
//! When adding nodes to the database, the workflow is the following:
//!
//! 1. Add the node to the osm_trees table.
//! 2. Check if there is a local tree with that node id.
//! 3. If not found, find a tree within 10 meters and link them.
//! 4. If a local tree is found, update it.

use crate::common::database::repositories::*;
use crate::infra::config::Config;
use crate::infra::overpass::OverpassClient;
use crate::infra::queue::Queue;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{error, info};
use std::sync::Arc;

const DEFAULT_STATE: &str = "healthy";

pub struct OsmReaderService {
    trees: Arc<TreeRepository>,
    overpass_client: Arc<OverpassClient>,
    queue: Arc<Queue>,
    user_id: u64,
    osm_trees: Arc<OsmTreeRepository>,
}

impl OsmReaderService {
    /**
     * Pull new nodes from the OSM database and add them to the local database.
     */
    pub async fn pull_trees(&self) -> Result<()> {
        info!("Running OSM reader service.");

        let doc = match self.overpass_client.query().await {
            Ok(doc) => doc,

            Err(e) => {
                error!("Error querying OSM: {e:?}");
                return Err(Error::OsmExchange);
            }
        };

        let mut added: u64 = 0;
        let mut updated: u64 = 0;

        for node in doc.iter() {
            if let Some(old) = self.osm_trees.get(node.id).await? {
                if old != *node {
                    self.osm_trees.update(node).await?;
                    info!("OSM node {} updated in the database.", node.id);
                    updated += 1;
                }
            } else {
                self.osm_trees.add(node).await?;
                info!("OSM node {} added to the database.", node.id);
                added += 1;
            }
        }

        info!(
            "Found {} OSM nodes, {} added, {} updated.",
            doc.len(),
            added,
            updated
        );

        Ok(())
    }

    /**
     * Match known OSM trees to local trees.
     */
    pub async fn match_trees(&self) -> Result<()> {
        let mut added: u64 = 0;
        let mut updated: u64 = 0;

        for node in self.osm_trees.all().await? {
            if self.tree_exists(&node).await? {
                continue;
            }

            if self.tree_updated(&node).await? {
                updated += 1;
                continue;
            }

            self.add_local_tree(&node).await?;
            added += 1;
        }

        info!("Matched OSM trees: {added} added, {updated} updated.");

        Ok(())
    }

    async fn tree_exists(&self, node: &OsmTreeRecord) -> Result<bool> {
        Ok(self.trees.get_by_osm_id(node.id).await?.is_some())
    }

    async fn tree_updated(&self, node: &OsmTreeRecord) -> Result<bool> {
        let closest = match self.find_closest_available_tree(node.lat, node.lon).await? {
            Some(tree) => tree,
            None => return Ok(false),
        };

        self.trees
            .update_osm_id(closest.id, node.id, self.user_id)
            .await?;

        Ok(true)
    }

    async fn add_local_tree(&self, node: &OsmTreeRecord) -> Result<TreeRecord> {
        let now = get_timestamp();

        let tree = TreeRecord {
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
            added_by: self.user_id,
            ..Default::default()
        };

        self.trees.add(&tree).await?;
        self.schedule_address_update(tree.id).await?;

        info!("Tree {} added from OSM node {}.", tree.id, node.id);

        Ok(tree)
    }

    async fn find_closest_available_tree(&self, lat: f64, lon: f64) -> Result<Option<TreeRecord>> {
        for tree in self.trees.get_close(lat, lon, 5.0).await? {
            if tree.state != "gone" && tree.osm_id.is_none() {
                return Ok(Some(tree));
            }
        }

        Ok(None)
    }

    async fn schedule_address_update(&self, tree_id: u64) -> Result<()> {
        let msg = UpdateTreeAddressMessage { id: tree_id };
        self.queue.push(&msg.encode()).await?;

        info!("Scheduled address update for tree {tree_id}");

        Ok(())
    }
}

impl Locatable for OsmReaderService {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            overpass_client: locator.get::<OverpassClient>()?,
            queue: locator.get::<Queue>()?,
            user_id: config.bot_user_id,
            osm_trees: locator.get::<OsmTreeRepository>()?,
        })
    }
}
