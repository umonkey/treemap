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

use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{error, info};
use std::sync::Arc;

const DEFAULT_STATE: &str = "healthy";

pub struct OsmReaderService {
    db: Arc<dyn DatabaseInterface>,
    overpass_client: Arc<OverpassClient>,
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
                error!("Error querying OSM: {:?}", e);
                return Err(Error::OsmExchange);
            }
        };

        let mut added: u64 = 0;
        let mut updated: u64 = 0;

        for node in doc.iter() {
            match self.db.get_osm_tree(node.id).await? {
                Some(prev) => {
                    if prev != *node {
                        self.db.add_osm_tree(node).await?;
                        info!("OSM node {} updated in the database.", node.id);
                        updated += 1;
                    }
                }

                None => {
                    self.db.add_osm_tree(node).await?;
                    info!("OSM node {} added to the database.", node.id);
                    added += 1;
                }
            };
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

        for node in self.db.find_osm_trees().await? {
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

        info!("Matched OSM trees: {} added, {} updated.", added, updated);

        Ok(())
    }

    async fn tree_exists(&self, node: &OsmTreeRecord) -> Result<bool> {
        Ok(self.db.get_tree_by_osm_id(node.id).await?.is_some())
    }

    async fn tree_updated(&self, node: &OsmTreeRecord) -> Result<bool> {
        let closest = match self.find_closest_available_tree(node.lat, node.lon).await? {
            Some(tree) => tree,
            None => return Ok(false),
        };

        self.db
            .update_tree(&TreeRecord {
                osm_id: Some(node.id),
                ..closest.clone()
            })
            .await?;

        info!("Tree {} linked to OSM node {}.", closest.id, node.id);

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
            notes: None,
            height: node.height,
            circumference: node.circumference,
            diameter: node.diameter_crown,
            state: DEFAULT_STATE.to_string(),
            added_at: now,
            updated_at: now,
            added_by: 0,
            thumbnail_id: None,
            year: None,
        };

        self.db.add_tree(&tree).await?;

        info!("Tree {} added from OSM node {}.", tree.id, node.id);

        Ok(tree)
    }

    async fn find_closest_available_tree(&self, lat: f64, lon: f64) -> Result<Option<TreeRecord>> {
        for tree in self.db.find_closest_trees(lat, lon, 5.0).await? {
            if tree.state != "gone" && tree.osm_id.is_none() {
                return Ok(Some(tree));
            }
        }

        Ok(None)
    }
}

impl Locatable for OsmReaderService {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let overpass_client = locator.get::<OverpassClient>()?;

        Ok(Self {
            db,
            overpass_client,
        })
    }
}
