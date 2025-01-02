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
    pub async fn get(&self, id: u64) -> Result<TreeRecord> {
        let query = SelectQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(id as i64))]),
            ..Default::default()
        };

        match self.db.get_record(query).await {
            Ok(Some(props)) => TreeRecord::from_attributes(&props),
            Ok(None) => Err(Error::TreeNotFound),
            Err(err) => {
                error!("Error reading a tree: {}", err);
                Err(err)
            }
        }
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
        let old = self.get(tree.id).await?;

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
