use crate::common::database::repositories::*;
use crate::domain::tree::Tree;
use crate::domain::tree::TreeRepository;
use crate::infra::config::Config;
use crate::infra::osm::OsmClient;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use log::info;
use std::sync::Arc;

const MAX_CHANGES: usize = 100;

// Don't push trees younger than 10 minutes, let users finish their surveys.
const MIN_AGE: u64 = 600;

pub struct OsmPushHandler {
    osm: Arc<OsmClient>,
    osm_trees: Arc<OsmTreeRepository>,
    trees: Arc<TreeRepository>,
    user_id: u64,
}

impl OsmPushHandler {
    pub async fn handle(&self) -> Result<()> {
        self.push_new_trees().await?;
        Ok(())
    }

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

    fn shall_add(&self, tree: &Tree) -> bool {
        if tree.osm_id.is_some() {
            return false;
        }

        if !tree.is_existing() {
            return false;
        }

        true
    }

    async fn get_new_trees(&self) -> Result<Vec<Tree>> {
        let mut res = Vec::new();
        let max_ts = get_timestamp() - MIN_AGE;

        for tree in self.trees.all().await? {
            if !self.shall_add(&tree) {
                continue;
            }

            if tree.added_at > max_ts {
                continue;
            }

            res.push(tree);

            if res.len() == MAX_CHANGES {
                break;
            }
        }

        Ok(res)
    }
}

impl Locatable for OsmPushHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        Ok(Self {
            osm: locator.get::<OsmClient>()?,
            osm_trees: locator.get::<OsmTreeRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            user_id: config.bot_user_id,
        })
    }
}
