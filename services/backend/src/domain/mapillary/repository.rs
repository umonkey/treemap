use crate::domain::mapillary::{
    MapillaryImage, MapillarySequence, MapillarySequenceSummary, MapillaryTree,
};
use crate::domain::tree::Bounds;
use crate::infra::database::{
    Database, DeleteQuery, InsertQuery, ReplaceQuery, SelectQuery, Value,
};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const IMAGES_TABLE: &str = "mapillary_images";
const SEQUENCES_TABLE: &str = "mapillary_sequences";
const TREES_TABLE: &str = "mapillary_trees";

pub struct MapillaryRepository {
    db: Arc<Database>,
}

impl MapillaryRepository {
    pub async fn add_image(&self, image: &MapillaryImage) -> Result<()> {
        let query = InsertQuery::new(IMAGES_TABLE).with_values(image.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn find_trees_by_image_id(&self, image_id: &str) -> Result<Vec<MapillaryTree>> {
        let query = SelectQuery::new(TREES_TABLE)
            .with_condition("image_id", Value::from(image_id.to_string()));

        let records = self.db.get_records(query).await?;
        records.iter().map(MapillaryTree::from_attributes).collect()
    }

    pub async fn add_tree(&self, tree: &MapillaryTree) -> Result<()> {
        let query = InsertQuery::new(TREES_TABLE).with_values(tree.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn delete_trees_by_image_id(&self, image_id: &str) -> Result<()> {
        let query = DeleteQuery::new(TREES_TABLE)
            .with_condition("image_id", Value::from(image_id.to_string()));

        self.db.delete(query).await?;
        Ok(())
    }

    pub async fn find_images_by_bounds(&self, bounds: Bounds) -> Result<Vec<MapillaryImage>> {
        let sql = format!(
            "SELECT i.* FROM `{}` i INNER JOIN `{}` s ON i.sequence_id = s.id WHERE i.`lat` <= ? AND i.lat >= ? AND i.lon <= ? AND i.lon >= ? AND s.hidden = 0",
            IMAGES_TABLE, SEQUENCES_TABLE
        );

        let params = &[
            Value::from(bounds.n),
            Value::from(bounds.s),
            Value::from(bounds.e),
            Value::from(bounds.w),
        ];

        let records = self.db.fetch_sql(&sql, params).await?;
        records
            .iter()
            .map(MapillaryImage::from_attributes)
            .collect()
    }

    pub async fn find_trees_with_location_by_bounds(
        &self,
        bounds: Bounds,
    ) -> Result<Vec<(MapillaryTree, f64, f64, f64)>> {
        let sql = format!(
            "SELECT t.*, i.lat, i.lon, i.compass_angle FROM `{}` t INNER JOIN `{}` i ON t.image_id = i.id WHERE i.`lat` <= ? AND i.lat >= ? AND i.lon <= ? AND i.lon >= ?",
            TREES_TABLE, IMAGES_TABLE
        );

        let params = &[
            Value::from(bounds.n),
            Value::from(bounds.s),
            Value::from(bounds.e),
            Value::from(bounds.w),
        ];

        let records = self.db.fetch_sql(&sql, params).await?;
        let mut res = Vec::new();

        for record in records {
            let tree = MapillaryTree::from_attributes(&record)?;
            let lat = record.require_f64("lat")?;
            let lon = record.require_f64("lon")?;
            let compass_angle = record.require_f64("compass_angle")?;
            res.push((tree, lat, lon, compass_angle));
        }

        Ok(res)
    }

    pub async fn find_images_by_sequence(&self, sequence_id: &str) -> Result<Vec<MapillaryImage>> {
        let query = SelectQuery::new(IMAGES_TABLE)
            .with_condition("sequence_id", Value::from(sequence_id.to_string()))
            .with_order("captured_at");

        let records = self.db.get_records(query).await?;
        records
            .iter()
            .map(MapillaryImage::from_attributes)
            .collect()
    }

    pub async fn add_sequence(&self, sequence: &MapillarySequence) -> Result<()> {
        let query = ReplaceQuery::new(SEQUENCES_TABLE).with_values(sequence.to_attributes());
        self.db.replace(query).await?;
        Ok(())
    }

    pub async fn find_sequence(&self, id: &str) -> Result<Option<MapillarySequence>> {
        let query =
            SelectQuery::new(SEQUENCES_TABLE).with_condition("id", Value::from(id.to_string()));

        let records = self.db.get_records(query).await?;
        if let Some(record) = records.first() {
            Ok(Some(MapillarySequence::from_attributes(record)?))
        } else {
            Ok(None)
        }
    }

    pub async fn find_sequences_by_bounds(&self, bounds: Bounds) -> Result<Vec<MapillarySequence>> {
        // Query sequences whose bounding box intersects with the requested bounds.
        let sql = format!(
            "SELECT * FROM `{}` WHERE `min_lat` <= ? AND `max_lat` >= ? AND `min_lon` <= ? AND `max_lon` >= ? AND `hidden` = 0",
            SEQUENCES_TABLE
        );

        let params = &[
            Value::from(bounds.n),
            Value::from(bounds.s),
            Value::from(bounds.e),
            Value::from(bounds.w),
        ];

        let records = self.db.fetch_sql(&sql, params).await?;
        records
            .iter()
            .map(MapillarySequence::from_attributes)
            .collect()
    }

    pub async fn find_all_sequences(&self) -> Result<Vec<MapillarySequenceSummary>> {
        let sql = format!(
            "SELECT id, captured_at, image_count, hidden FROM `{}` ORDER BY captured_at DESC",
            SEQUENCES_TABLE
        );

        let records = self.db.fetch_sql(&sql, &[]).await?;
        records
            .iter()
            .map(MapillarySequenceSummary::from_attributes)
            .collect()
    }
}

impl Injectable for MapillaryRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
