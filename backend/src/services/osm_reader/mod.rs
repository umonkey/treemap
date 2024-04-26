/**
 * This module reads data from OSM and puts it into the database.
 *
 * The workflow is the following:
 *
 * 1. Query OSM for data.
 * 2. Parse the data.
 * 3. Filter out the data that is not trees.
 * 4. Add the data to the database.
 *
 * When adding nodes to the database, the workflow is the following:
 *
 * 1. Add the node to the osm_trees table.
 * 2. Check if there is a local tree with that node id.
 * 3. If not found, find a tree within 10 meters and link them.
 * 4. If a local tree is found, update it.
 */

use log::{debug, error, info};
use std::sync::Arc;

use crate::types::{Error, OsmTreeRecord, Result, TreeRecord};
use crate::services::{get_database, Database, OverpassClient};
use crate::utils::{get_timestamp, get_unique_id};

const DEFAULT_STATE: &str = "healthy";

pub struct OsmReaderService {
    db: Arc<dyn Database>,
    overpass_client: OverpassClient,
}

impl OsmReaderService {
    pub async fn init() -> Result<Self> {
        let db = get_database().await?;

        Ok(Self {
            db: db.clone(),
            overpass_client: OverpassClient::init(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Running OSM reader service.");

        let doc = match self.overpass_client.query().await {
            Ok(doc) => doc,

            Err(e) => {
                error!("Error querying OSM: {:?}", e);
                return Err(Error::OsmExchange);
            },
        };

        debug!("Going to add {} nodes to the database.", doc.len());

        for node in doc.iter() {
            if self.process_node(node).await.is_err() {
                debug!("Oops, could not add node {}.", node.id);
            }
        }

        debug!("Checked {} OSM nodes.", doc.len());

        Ok(())
    }

    async fn process_node(&self, node: &OsmTreeRecord) -> Result<()> {
        if self.db.get_osm_tree(node.id).await?.is_some() {
            debug!("Node {} already exists in the database.", node.id);
            return Ok(());
        }

        self.db.add_osm_tree(node).await?;
        info!("OSM node {} added to the database.", node.id);

        if let Some(local) = self.find_local_tree(node).await? {
            self.update_tree(&local, node).await?;
        }

        Ok(())
    }

    async fn update_tree(&self, tree: &TreeRecord, node: &OsmTreeRecord) -> Result<()> {
        if !self.has_changes(tree, node) {
            debug!("Tree {} has no changes.", tree.id);
            return Ok(());
        }

        let new = TreeRecord {
            lat: node.lat,
            lon: node.lon,
            species: node.get_species(),
            height: node.height,
            circumference: node.circumference,
            diameter: node.diameter_crown,
            ..tree.clone()
        };

        self.db.update_tree(&new).await?;

        debug!("Updated tree {} with OSM data from node {}.", tree.id, node.id);

        Ok(())
    }

    fn has_changes(&self, tree: &TreeRecord, node: &OsmTreeRecord) -> bool {
        tree.species != node.get_species()
            || tree.height != node.height
            || tree.circumference != node.circumference
            || tree.diameter != node.diameter_crown
    }

    async fn find_local_tree(&self, node: &OsmTreeRecord) -> Result<Option<TreeRecord>> {
        // There is a linked tree.
        if let Some(tree) = self.db.get_tree_by_osm_id(node.id).await? {
            return Ok(Some(tree));
        }

        // There is a very close tree.
        if let Some(tree) = self.db.find_closest_tree(node.lat, node.lon, 5.0).await? {
            self.db.update_tree(&TreeRecord {
                osm_id: Some(node.id),
                ..tree.clone()
            }).await?;

            info!("Tree {} linked to OSM node {}.", tree.id, node.id);

            return Ok(Some(tree));
        }

        Ok(Some(self.add_local_tree(node).await?))
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
        };

        self.db.add_tree(&tree).await?;

        info!("Tree {} added from OSM node {}.", tree.id, node.id);

        Ok(tree)
    }
}
