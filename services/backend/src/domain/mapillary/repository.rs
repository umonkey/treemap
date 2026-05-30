use crate::domain::mapillary::MapillaryImage;
use crate::infra::database::{Database, InsertQuery, SelectQuery};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "mapillary_images";

pub struct MapillaryRepository {
    db: Arc<Database>,
}

impl MapillaryRepository {
    pub async fn add(&self, image: &MapillaryImage) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(image.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn find_all(&self) -> Result<Vec<MapillaryImage>> {
        let query = SelectQuery::new(TABLE).with_order("captured_at");
        let records = self.db.get_records(query).await?;
        records
            .iter()
            .map(MapillaryImage::from_attributes)
            .collect()
    }
}

impl Injectable for MapillaryRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
