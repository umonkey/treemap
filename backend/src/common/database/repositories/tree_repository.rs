use crate::common::database::queries::*;
use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::error;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "trees";

pub struct TreeRepository {
    db: Arc<dyn DatabaseInterface>,
    props: Arc<PropRepository>,
}

impl TreeRepository {
    pub async fn all(&self) -> Result<Vec<TreeRecord>> {
        self.query_multiple(SelectQuery::new(TABLE)).await
    }

    pub async fn get(&self, id: u64) -> Result<Option<TreeRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_multiple(&self, ids: &[u64]) -> Result<Vec<TreeRecord>> {
        let mut trees: Vec<TreeRecord> = Vec::new();

        for id in ids {
            if let Some(tree) = self.get(*id).await? {
                trees.push(tree);
            }
        }

        Ok(trees)
    }

    pub async fn get_last_by_user(&self, user_id: u64) -> Result<Option<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("added_by", Value::from(user_id as i64))
            .with_order_desc("created_at")
            .with_limit(1);

        self.query_single(query).await
    }

    pub async fn get_by_osm_id(&self, id: u64) -> Result<Option<TreeRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("osm_id", Value::from(id as i64));

        self.query_single(query).await
    }

    // FIXME: use a proper query
    pub async fn get_by_bounds(&self, bounds: Bounds) -> Result<Vec<TreeRecord>> {
        let trees = self
            .all()
            .await?
            .into_iter()
            .filter(|tree| {
                tree.lat <= bounds.n
                    && tree.lat >= bounds.s
                    && tree.lon <= bounds.e
                    && tree.lon >= bounds.w
            })
            .collect();

        Ok(trees)
    }

    pub async fn get_recently_created(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("created_at")
            .with_limit(count)
            .with_offset(skip);

        self.query_multiple(query).await
    }

    pub async fn get_recently_updated(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("updated_at")
            .with_limit(count)
            .with_offset(skip);

        self.query_multiple(query).await
    }

    pub async fn get_close(&self, lat: f64, lon: f64, distance: f64) -> Result<Vec<TreeRecord>> {
        let delta = distance / 111_111.0; // meters per degree

        let bounds = Bounds {
            n: lat + delta,
            s: lat - delta,
            e: lon + delta,
            w: lon - delta,
        };

        self.get_by_bounds(bounds).await
    }

    pub async fn get_with_no_address(&self) -> Result<Vec<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("address", Value::Null)
            .with_order_desc("updated_at");

        self.query_multiple(query).await
    }

    pub async fn get_top_height(&self, count: u64) -> Result<Vec<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("height")
            .with_limit(count);

        self.query_multiple(query).await
    }

    pub async fn get_top_circumference(&self, count: u64) -> Result<Vec<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("circumference")
            .with_limit(count);

        self.query_multiple(query).await
    }

    pub async fn get_top_diameter(&self, count: u64) -> Result<Vec<TreeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("diameter")
            .with_limit(count);

        self.query_multiple(query).await
    }

    pub async fn add(&self, tree: &TreeRecord) -> Result<()> {
        self.db
            .add_record(InsertQuery {
                table_name: TABLE.to_string(),
                attributes: tree.to_attributes(),
            })
            .await?;

        self.log_changes(&TreeRecord::default(), tree, tree.added_by)
            .await
    }

    pub async fn update(&self, tree: &TreeRecord, user_id: u64) -> Result<()> {
        let old = self.get(tree.id).await?.ok_or_else(|| {
            error!("Error updating a tree: tree not found");
            Error::TreeNotFound
        })?;

        let query = UpdateQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(tree.id as i64))]),
            attributes: tree.to_attributes(),
        };

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {}", e);
            e
        })?;

        self.log_changes(&old, tree, user_id).await
    }

    pub async fn update_thumbnail(
        &self,
        tree_id: u64,
        thumbnail_id: u64,
        user_id: u64,
    ) -> Result<()> {
        let query = UpdateQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(tree_id as i64))]),
            attributes: Attributes::from(&[(
                "thumbnail_id".to_string(),
                Value::from(thumbnail_id as i64),
            )]),
        };

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {}", e);
            e
        })?;

        self.add_tree_prop(tree_id, "thumbnail_id", &thumbnail_id.to_string(), user_id)
            .await
    }

    pub async fn update_osm_id(&self, tree_id: u64, osm_id: u64) -> Result<()> {
        let query = UpdateQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(tree_id as i64))]),
            attributes: Attributes::from(&[("osm_id".to_string(), Value::from(osm_id as i64))]),
        };

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {}", e);
            e
        })?;

        Ok(())
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<TreeRecord>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(TreeRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a tree: {}", err);
                Err(err)
            }
        }
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<TreeRecord>> {
        let records = self.db.get_records(query).await?;

        Ok(records
            .iter()
            .map(|props| TreeRecord::from_attributes(props).unwrap())
            .collect())
    }

    async fn log_changes(&self, old: &TreeRecord, new: &TreeRecord, user_id: u64) -> Result<()> {
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

        if old.lat != new.lat {
            self.add_tree_prop(new.id, "lat", &new.lat.to_string(), user_id)
                .await?;
        }

        if old.lon != new.lon {
            self.add_tree_prop(new.id, "lon", &new.lon.to_string(), user_id)
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
}

impl Locatable for TreeRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let props = locator.get::<PropRepository>()?;
        Ok(Self { db, props })
    }
}
