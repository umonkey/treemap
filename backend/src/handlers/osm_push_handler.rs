use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const MAX_CHANGES: usize = 100;

pub struct OsmPushHandler {
    osm: Arc<OsmClient>,
    osm_trees: Arc<OsmTreeRepository>,
    trees: Arc<TreeRepository>,
}

impl OsmPushHandler {
    pub async fn handle(&self) -> Result<()> {
        self.push_new_trees().await?;
        Ok(())
    }

    pub async fn push_new_trees(&self) -> Result<()> {
        let changeset = self
            .osm
            .create_changeset("Adding new trees found during surveys.")
            .await?;

        for tree in self.get_new_trees().await? {
            let osm_id = self.osm.create_tree(changeset, &tree).await?;

            self.trees.update_osm_id(tree.id, osm_id).await?;

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

    fn shall_add(&self, tree: &TreeRecord) -> bool {
        if tree.osm_id.is_some() {
            return false;
        }

        if !tree.is_existing() {
            return false;
        }

        true
    }

    async fn get_new_trees(&self) -> Result<Vec<TreeRecord>> {
        let mut res = Vec::new();

        for tree in self.trees.all().await? {
            if self.shall_add(&tree) {
                res.push(tree);

                if res.len() == MAX_CHANGES {
                    break;
                }
            }
        }

        Ok(res)
    }
}

impl Locatable for OsmPushHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            osm: locator.get::<OsmClient>()?,
            osm_trees: locator.get::<OsmTreeRepository>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
