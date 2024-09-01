/**
 * The tree service performs updates to the trees database.
 *
 * Reading is straight-forward, updates also log individual property changes
 * to be able to track changes over time.
 */
use log::debug;
use std::sync::Arc;

use crate::services::Database;
use crate::types::{
    AddTreeRequest, Error, GetTreesRequest, MoveTreeRequest, Result, SearchQuery, TreeList,
    TreeRecord, TreeStats, UpdateTreeRequest,
};
use crate::utils::{get_timestamp, get_unique_id};

pub struct Trees {
    db: Arc<dyn Database>,
}

impl Trees {
    pub async fn new(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn add_trees(&self, req: AddTreeRequest) -> Result<Vec<TreeRecord>> {
        let now = get_timestamp();

        let mut trees: Vec<TreeRecord> = Vec::new();

        for point in req.points {
            let tree = TreeRecord {
                id: get_unique_id()?,
                osm_id: None,
                lat: point.lat,
                lon: point.lon,
                species: req.species.clone(),
                notes: req.notes.clone(),
                height: Self::scale_cm_to_m(req.height),
                circumference: Self::scale_cm_to_m(req.circumference),
                diameter: Self::scale_cm_to_m(req.diameter),
                state: req.state.to_string(),
                added_at: now,
                updated_at: now,
                added_by: req.user_id,
                thumbnail_id: None,
            };

            debug!(
                "Adding tree at ({}, {}) with species '{}'.",
                tree.lat, tree.lon, tree.species
            );

            self.db.add_tree(&tree).await?;

            trees.push(tree);
        }

        Ok(trees)
    }

    pub async fn update_tree(&self, req: UpdateTreeRequest) -> Result<TreeRecord> {
        let now = get_timestamp();

        let old = self.get_tree(req.id).await?;

        let new = TreeRecord {
            id: req.id,
            osm_id: old.osm_id,
            lat: old.lat,
            lon: old.lon,
            species: req.species,
            notes: req.notes,
            height: Self::scale_cm_to_m(req.height),
            circumference: Self::scale_cm_to_m(req.circumference),
            diameter: Self::scale_cm_to_m(req.diameter),
            state: req.state,
            added_at: old.added_at,
            updated_at: now,
            added_by: old.added_by,
            thumbnail_id: old.thumbnail_id,
        };

        debug!("Updating tree: {:?}", new);

        self.db.update_tree(&new).await?;

        Ok(new)
    }

    pub async fn move_tree(&self, req: &MoveTreeRequest) -> Result<()> {
        let id = req.id;
        let lat = req.lat;
        let lon = req.lon;

        debug!("Moving tree {} to ({}, {}).", id, lat, lon);

        self.db.move_tree(id, lat, lon).await?;

        Ok(())
    }

    pub async fn get_trees(&self, request: &GetTreesRequest) -> Result<TreeList> {
        let mut trees = self.db.get_trees(request.bounds()).await?;

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            trees.retain(|t| query.r#match(t));
        }

        Ok(TreeList::from_trees(trees))
    }

    pub async fn get_tree(&self, id: u64) -> Result<TreeRecord> {
        debug!("Getting details for tree {}.", id);

        match self.db.get_tree(id).await? {
            Some(tree) => Ok(tree),

            None => Err(Error::TreeNotFound),
        }
    }

    pub async fn get_tree_stats(&self) -> Result<TreeStats> {
        Ok(TreeStats {
            count: self.db.count_trees().await?,
        })
    }

    pub async fn get_last_tree_by_user(&self, user_id: u64) -> Result<Option<TreeRecord>> {
        debug!("Getting last tree for user {}.", user_id);
        self.db.get_last_tree_by_user(user_id).await
    }

    fn scale_cm_to_m(value: Option<f64>) -> Option<f64> {
        let mut value: f64 = match value {
            Some(v) => v,
            None => return None,
        };

        // contemporary max alive 116.07m: https://en.wikipedia.org/wiki/List_of_superlative_trees#Tallest
        // contemporary max alive 36.2m: https://en.wikipedia.org/wiki/List_of_superlative_trees#Stoutest
        // contemporary max alive 72.8: https://en.wikipedia.org/wiki/List_of_superlative_trees#Broadest excluding banyan
        if value.fract() == 0.0 && value > 9 {
            value /= 100.0;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::SqliteDatabase;
    use crate::types::LatLon;
    use env_logger;
    use std::env;

    async fn setup(data: Option<String>) -> Result<Trees> {
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        let db = SqliteDatabase::new().await?;
        db.execute(include_str!("./fixtures/init.sql").to_string())
            .await?;

        if let Some(init_script) = data {
            db.execute(init_script).await?;
        }

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let dbh: Arc<dyn Database> = Arc::new(db);
        Ok(Trees::new(&dbh).await)
    }

    #[tokio::test]
    async fn test_get_no_trees() -> Result<()> {
        let service = setup(None).await?;

        let trees = service
            .get_trees(&GetTreesRequest {
                n: 90.0,
                e: 180.0,
                s: -90.0,
                w: -180.0,
                search: None,
            })
            .await?;

        assert_eq!(trees.trees.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_some_trees() -> Result<()> {
        let script = include_str!("./fixtures/some-trees.sql");

        let service = setup(Some(script.to_string()))
            .await
            .expect("Error initializing database");

        let trees = service
            .get_trees(&GetTreesRequest {
                n: 90.0,
                e: 180.0,
                s: -90.0,
                w: -180.0,
                search: None,
            })
            .await
            .expect("Error getting trees");

        assert_eq!(trees.trees.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_tree_minimal() {
        let service = setup(None).await.expect("Error initializing the service");

        let trees = service
            .add_trees(AddTreeRequest {
                points: vec![LatLon {
                    lat: 12.34,
                    lon: 56.78,
                }],
                species: "Oak".to_string(),
                notes: None,
                height: None,
                circumference: None,
                diameter: None,
                state: "healthy".to_string(),
                user_id: 3,
            })
            .await
            .expect("Error adding tree");

        assert_eq!(trees.len(), 1);

        let tree = &trees[0];

        debug!("Tree added: {:?}", tree);

        let tree = service
            .get_tree(tree.id)
            .await
            .expect("Error reading a tree that was just added");

        assert_eq!(tree.lat, 12.34);
        assert_eq!(tree.lon, 56.78);
        assert_eq!(tree.species, "Oak");
        assert!(tree.notes.is_none(), "notes should be empty");
        assert!(tree.height.is_none(), "height should be empty");
        assert!(
            tree.circumference.is_none(),
            "circumference should be empty"
        );
        assert!(tree.diameter.is_none(), "diameter should be empty");
        assert_eq!(tree.state, "healthy");
        assert_eq!(tree.added_by, 3);
    }

    #[tokio::test]
    async fn test_add_tree_full() {
        let service = setup(None).await.expect("Error initializing the service");

        let trees = service
            .add_trees(AddTreeRequest {
                points: vec![LatLon {
                    lat: 12.34,
                    lon: 56.78,
                }],
                species: "Quercus".to_string(),
                notes: Some("Oak".to_string()),
                height: Some(12.3),
                circumference: Some(3.4),
                diameter: Some(15.0),
                state: "healthy".to_string(),
                user_id: 3,
            })
            .await
            .expect("Error adding tree");

        assert_eq!(trees.len(), 1);

        let tree = service
            .get_tree(trees[0].id)
            .await
            .expect("Error reading a tree that was just added");

        assert_eq!(tree.lat, 12.34);
        assert_eq!(tree.lon, 56.78);
        assert_eq!(tree.species, "Quercus");
        assert_eq!(tree.notes.expect("notes not set"), "Oak");
        assert_eq!(tree.height.expect("height not set"), 12.3);
        assert_eq!(tree.circumference.expect("circumference not set"), 3.4);
        assert_eq!(tree.diameter.expect("diameter not set"), 15.0);
        assert_eq!(tree.state, "healthy");
        assert_eq!(tree.added_by, 3);
    }

    #[tokio::test]
    async fn test_keep_circumference() {
        let service = setup(None).await.expect("Error initializing the service");

        let trees = service
            .add_trees(AddTreeRequest {
                points: vec![LatLon {
                    lat: 12.34,
                    lon: 56.78,
                }],
                species: "Quercus".to_string(),
                notes: Some("Oak".to_string()),
                height: Some(12.3),
                circumference: Some(2.0),
                diameter: Some(15.0),
                state: "healthy".to_string(),
                user_id: 3,
            })
            .await
            .expect("Error adding tree");

        assert_eq!(2.0, trees[0].circumference.unwrap());
    }

    /**
     * When circumference is a whole number greater than 3, it is assumed to be in cm.
     */
    #[tokio::test]
    async fn test_convert_circumference() {
        let service = setup(None).await.expect("Error initializing the service");

        let trees = service
            .add_trees(AddTreeRequest {
                points: vec![LatLon {
                    lat: 12.34,
                    lon: 56.78,
                }],
                species: "Quercus".to_string(),
                notes: Some("Oak".to_string()),
                height: Some(12.3),
                circumference: Some(123.0),
                diameter: Some(15.0),
                state: "healthy".to_string(),
                user_id: 3,
            })
            .await
            .expect("Error adding tree");

        assert_eq!(1.23, trees[0].circumference.unwrap());
    }
}
