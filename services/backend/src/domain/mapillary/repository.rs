use crate::domain::mapillary::{MapillaryImage, MapillarySequence};
use crate::domain::tree::Bounds;
use crate::infra::database::{Database, InsertQuery, ReplaceQuery, SelectQuery, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const IMAGES_TABLE: &str = "mapillary_images";
const SEQUENCES_TABLE: &str = "mapillary_sequences";

pub struct MapillaryRepository {
    db: Arc<Database>,
}

impl MapillaryRepository {
    pub async fn add_image(&self, image: &MapillaryImage) -> Result<()> {
        let query = InsertQuery::new(IMAGES_TABLE).with_values(image.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn find_images_by_bounds(&self, bounds: Bounds) -> Result<Vec<MapillaryImage>> {
        let sql = format!(
            "SELECT * FROM `{}` WHERE `lat` <= ? AND lat >= ? AND lon <= ? AND lon >= ?",
            IMAGES_TABLE
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

    pub async fn find_sequences_by_bounds(&self, bounds: Bounds) -> Result<Vec<MapillarySequence>> {
        // Query sequences whose bounding box intersects with the requested bounds.
        let sql = format!(
            "SELECT * FROM `{}` WHERE `min_lat` <= ? AND `max_lat` >= ? AND `min_lon` <= ? AND `max_lon` >= ?",
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
}

impl Injectable for MapillaryRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
