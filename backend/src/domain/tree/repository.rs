use super::models::Tree;
use super::schemas::*;
use crate::domain::prop::{PropRecord, PropRepository};
use crate::infra::database::{CountQuery, IncrementQuery, InsertQuery, SelectQuery, UpdateQuery};
use crate::infra::database::{Database, Value};
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use log::{debug, error, info};
use std::sync::Arc;

const TABLE: &str = "trees";

pub struct TreeRepository {
    db: Arc<Database>,
    props: Arc<PropRepository>,
}

impl TreeRepository {
    pub async fn all(&self) -> Result<Vec<Tree>> {
        self.query_multiple(SelectQuery::new(TABLE)).await
    }

    pub async fn count(&self) -> Result<u64> {
        let query = CountQuery::new(TABLE);
        self.db.count(query).await
    }

    pub async fn get(&self, id: u64) -> Result<Option<Tree>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_multiple(&self, ids: &[u64]) -> Result<Vec<Tree>> {
        let mut trees: Vec<Tree> = Vec::new();

        for id in ids {
            if let Some(tree) = self.get(*id).await? {
                trees.push(tree);
            }
        }

        Ok(trees)
    }

    pub async fn get_last_by_user(&self, user_id: u64) -> Result<Option<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("added_by", Value::from(user_id as i64))
            .with_order_desc("added_at")
            .with_limit(1);

        self.query_single(query).await
    }

    pub async fn get_by_osm_id(&self, id: u64) -> Result<Option<Tree>> {
        let query = SelectQuery::new(TABLE).with_condition("osm_id", Value::from(id as i64));

        self.query_single(query).await
    }

    pub async fn get_by_bounds(&self, bounds: Bounds) -> Result<Vec<Tree>> {
        let query = "SELECT * FROM `trees` WHERE `lat` <= ? AND lat >= ? AND lon <= ? AND lon >= ?";

        let params = &[
            Value::from(bounds.n),
            Value::from(bounds.s),
            Value::from(bounds.e),
            Value::from(bounds.w),
        ];

        self.fetch(query, params).await
    }

    pub async fn get_recently_created(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("added_at")
            .with_limit(count)
            .with_offset(skip);

        self.query_multiple(query).await
    }

    pub async fn get_recently_updated(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("updated_at")
            .with_limit(count)
            .with_offset(skip);

        self.query_multiple(query).await
    }

    pub async fn get_close(&self, lat: f64, lon: f64, distance: f64) -> Result<Vec<Tree>> {
        let delta = distance / 111_111.0; // meters per degree

        let bounds = Bounds {
            n: lat + delta,
            s: lat - delta,
            e: lon + delta,
            w: lon - delta,
        };

        self.get_by_bounds(bounds).await
    }

    pub async fn get_with_no_address(&self) -> Result<Vec<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("address", Value::Null)
            .with_order_desc("updated_at");

        self.query_multiple(query).await
    }

    pub async fn get_top_height(&self, count: u64) -> Result<Vec<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("height")
            .with_limit(count);

        self.query_multiple(query).await
    }

    pub async fn get_top_circumference(&self, count: u64) -> Result<Vec<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("circumference")
            .with_limit(count);

        self.query_multiple(query).await
    }

    pub async fn get_top_diameter(&self, count: u64) -> Result<Vec<Tree>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("diameter")
            .with_limit(count);

        self.query_multiple(query).await
    }

    /// Add a new tree.
    ///
    /// Makes sure that there's no existing tree with the same coordinates.
    pub async fn add(&self, tree: &Tree) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(tree.to_attributes());

        self.db.add_record(query).await?;

        self.log_changes(&Tree::default(), tree, tree.added_by)
            .await
    }

    pub async fn update(&self, tree: &Tree, user_id: u64) -> Result<Tree> {
        let old = self.get(tree.id).await?.ok_or_else(|| {
            error!("Error updating a tree: tree not found");
            Error::TreeNotFound
        })?;

        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree.id as i64))
            .with_values(tree.to_attributes())
            .with_value("updated_at", Value::from(get_timestamp() as i64));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {e}");
            e
        })?;

        self.log_changes(&old, tree, user_id).await?;

        Ok(tree.clone())
    }

    pub async fn replace(&self, old_id: u64, new_id: u64, user_id: u64) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(old_id as i64))
            .with_value("state", Value::from("gone".to_string()))
            .with_value("replaced_by", Value::from(new_id as i64));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {e}");
            e
        })?;

        self.add_tree_prop(old_id, "state", "gone", user_id).await?;
        self.add_tree_prop(old_id, "replaced_by", &new_id.to_string(), user_id)
            .await?;

        Ok(())
    }

    pub async fn r#move(&self, tree: &Tree, lat: f64, lon: f64, user_id: u64) -> Result<()> {
        let old = tree.clone();

        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree.id as i64))
            .with_value("updated_at", Value::from(get_timestamp() as i64))
            .with_value("lat", Value::from(lat))
            .with_value("lon", Value::from(lon));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {e}");
            e
        })?;

        let new = Tree {
            lat,
            lon,
            ..tree.clone()
        };

        self.log_changes(&old, &new, user_id).await
    }

    pub async fn update_thumbnail(
        &self,
        tree_id: u64,
        thumbnail_id: u64,
        user_id: u64,
    ) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree_id as i64))
            .with_value("updated_at", Value::from(get_timestamp() as i64))
            .with_value("thumbnail_id", Value::from(thumbnail_id as i64));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {e}");
            e
        })?;

        self.add_tree_prop(tree_id, "thumbnail_id", &thumbnail_id.to_string(), user_id)
            .await
    }

    pub async fn update_osm_id(&self, tree_id: u64, osm_id: u64, user_id: u64) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree_id as i64))
            .with_value("osm_id", Value::from(osm_id as i64));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {e}");
            e
        })?;

        self.add_tree_prop(tree_id, "osm_id", &osm_id.to_string(), user_id)
            .await?;

        info!("Tree {tree_id} linked to OSM node {osm_id} by user {user_id}.");

        Ok(())
    }

    pub async fn increment_likes(&self, tree_id: u64, value: i64) -> Result<()> {
        self.increment(tree_id, "like_count", value).await
    }

    pub async fn update_comment_count(&self, tree_id: u64, count: u64) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree_id as i64))
            .with_value("comment_count", Value::from(count as i64));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating comment count for a tree: {e}");
            e
        })?;

        debug!("Comment count for tree {tree_id} set to {count}");

        Ok(())
    }

    async fn increment(&self, tree_id: u64, key: &str, value: i64) -> Result<()> {
        let query = IncrementQuery::new(TABLE)
            .with_condition("id", Value::from(tree_id as i64))
            .with_key(key)
            .with_value(value);

        self.db.increment(query).await.map_err(|e| {
            error!("Error incrementing {key} for tree {tree_id}: {e}");
            e
        })?;

        Ok(())
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<Tree>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(Tree::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a tree: {err}");
                Err(err)
            }
        }
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<Tree>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| Tree::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }

    async fn log_changes(&self, old: &Tree, new: &Tree, user_id: u64) -> Result<()> {
        if old.species != new.species {
            self.add_tree_prop(new.id, "species", &new.species, user_id)
                .await?;
        }

        if old.osm_id != new.osm_id {
            let value = match new.osm_id {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "osm_id", &value, user_id)
                .await?;
        }

        if old.lat != new.lat || old.lon != new.lon {
            self.add_tree_prop(
                new.id,
                "location",
                format!("{},{}", new.lat, new.lon).as_str(),
                user_id,
            )
            .await?;
        }

        if old.species != new.species {
            self.add_tree_prop(new.id, "species", &new.species, user_id)
                .await?;
        }

        if old.notes != new.notes {
            let value = match &new.notes {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "notes", &value, user_id).await?;
        }

        if old.height != new.height {
            let value = match new.height {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "height", &value, user_id)
                .await?;
        }

        if old.circumference != new.circumference {
            let value = match new.circumference {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "circumference", &value, user_id)
                .await?;
        }

        if old.diameter != new.diameter {
            let value = match new.diameter {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "diameter", &value, user_id)
                .await?;
        }

        if old.state != new.state {
            self.add_tree_prop(new.id, "state", &new.state, user_id)
                .await?;
        }

        if old.thumbnail_id != new.thumbnail_id {
            let value = match new.thumbnail_id {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "thumbnail_id", &value, user_id)
                .await?;
        }

        if old.replaces != new.replaces {
            let value = match new.replaces {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "replaces", &value, user_id)
                .await?;
        }

        Ok(())
    }

    async fn add_tree_prop(
        &self,
        tree_id: u64,
        name: &str,
        value: &str,
        user_id: u64,
    ) -> Result<()> {
        self.props
            .add(&PropRecord {
                tree_id,
                name: name.to_string(),
                value: value.to_string(),
                added_by: user_id,
                ..Default::default()
            })
            .await
    }

    async fn fetch(&self, sql: &str, params: &[Value]) -> Result<Vec<Tree>> {
        let rows = self.db.sql(sql, params).await?;

        rows.iter()
            .map(|props| Tree::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for TreeRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        let props = locator.get::<PropRepository>()?;
        Ok(Self { db, props })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Arc<TreeRepository> {
        let locator = Locator::new();

        locator
            .get::<TreeRepository>()
            .expect("Error creating tree repository.")
    }

    #[tokio::test]
    async fn test_get_by_bounds() {
        let repo = setup();

        repo.add(&Tree {
            id: 1,
            lat: 20.0,
            lon: 20.0,
            ..Default::default()
        })
        .await
        .expect("Error adding a tree.");

        // (1) The tree is within bounds and should be returned.
        let res = repo
            .get_by_bounds(Bounds {
                n: 40.0,
                e: 40.0,
                s: 0.0,
                w: 0.0,
            })
            .await
            .expect("Error getting trees.");

        assert_eq!(1, res.len());

        // (2) The tree is outside bounds and the result should be empty.
        let res = repo
            .get_by_bounds(Bounds {
                n: 40.0,
                e: 40.0,
                s: 30.0,
                w: 30.0,
            })
            .await
            .expect("Error getting trees.");

        assert_eq!(0, res.len());
    }
}
