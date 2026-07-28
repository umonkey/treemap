use super::models::{Panorama, PanoramaHint, PanoramaImage};
use crate::domain::tree::Bounds;
use crate::infra::database::{Database, DeleteQuery, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "panoramas";
const IMAGES_TABLE: &str = "panoramas_images";
const HINTS_TABLE: &str = "panoramas_hints";

pub struct PanoramaRepository {
    db: Arc<Database>,
}

impl PanoramaRepository {
    pub async fn all(&self) -> Result<Vec<Panorama>> {
        let query = SelectQuery::new(TABLE).with_order_desc("created_at");
        let records = self.db.get_records(query).await?;
        records.iter().map(Panorama::from_attributes).collect()
    }

    pub async fn get(&self, id: u64) -> Result<Option<Panorama>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        match self.db.get_record(query).await? {
            Some(attrs) => Ok(Some(Panorama::from_attributes(&attrs)?)),
            None => Ok(None),
        }
    }

    pub async fn add(&self, panorama: &Panorama) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(panorama.to_attributes());
        self.db.add_record(query).await
    }

    pub async fn update(&self, id: u64, panorama: &Panorama) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_values(panorama.to_attributes());
        self.db.update(query).await?;
        Ok(())
    }

    pub async fn delete_images(&self, panorama_id: u64) -> Result<()> {
        let query = DeleteQuery::new(IMAGES_TABLE)
            .with_condition("panorama_id", Value::from(panorama_id as i64));
        self.db.delete(query).await?;
        Ok(())
    }

    pub async fn add_images(&self, images: &[PanoramaImage]) -> Result<()> {
        for image in images {
            let query = InsertQuery::new(IMAGES_TABLE).with_values(image.to_attributes());
            self.db.add_record(query).await?;
        }
        Ok(())
    }

    pub async fn get_image(&self, id: u64) -> Result<Option<PanoramaImage>> {
        let query = SelectQuery::new(IMAGES_TABLE).with_condition("id", Value::from(id as i64));
        match self.db.get_record(query).await? {
            Some(attrs) => Ok(Some(PanoramaImage::from_attributes(&attrs)?)),
            None => Ok(None),
        }
    }

    pub async fn get_images(&self, panorama_id: u64) -> Result<Vec<PanoramaImage>> {
        let query = SelectQuery::new(IMAGES_TABLE)
            .with_condition("panorama_id", Value::from(panorama_id as i64));
        let records = self.db.get_records(query).await?;
        records.iter().map(PanoramaImage::from_attributes).collect()
    }

    pub async fn find_by_bounds(&self, bounds: Bounds) -> Result<Vec<Panorama>> {
        let sql = format!(
            "SELECT * FROM `{}` WHERE `min_lat` <= ? AND `max_lat` >= ? AND `min_lon` <= ? AND `max_lon` >= ? AND `status` = 'SUCCESS' AND `visible` = 1",
            TABLE
        );

        let params = &[
            Value::from(bounds.n),
            Value::from(bounds.s),
            Value::from(bounds.e),
            Value::from(bounds.w),
        ];

        let records = self.db.fetch_sql(&sql, params).await?;
        records.iter().map(Panorama::from_attributes).collect()
    }

    pub async fn find_images_by_bounds(&self, bounds: Bounds) -> Result<Vec<(PanoramaImage, i64)>> {
        let sql = format!(
            "SELECT i.*, p.created_at FROM `{}` i INNER JOIN `{}` p ON i.panorama_id = p.id WHERE i.`lat` <= ? AND i.lat >= ? AND i.lng <= ? AND i.lng >= ? AND i.hidden = 0 AND p.status = 'SUCCESS' AND p.visible = 1",
            IMAGES_TABLE, TABLE
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
            let img = PanoramaImage::from_attributes(&record)?;
            let created_at = record.require_i64("created_at")?;
            res.push((img, created_at));
        }

        Ok(res)
    }

    pub async fn find_hints_with_location_by_bounds(
        &self,
        bounds: Bounds,
    ) -> Result<Vec<(PanoramaHint, f64, f64, f64)>> {
        let sql = format!(
            "SELECT h.*, i.lat, i.lng, i.heading FROM `{}` h INNER JOIN `{}` i ON h.image_id = i.id INNER JOIN `{}` p ON i.panorama_id = p.id WHERE i.`lat` <= ? AND i.lat >= ? AND i.lng <= ? AND i.lng >= ? AND i.hidden = 0 AND p.status = 'SUCCESS' AND p.visible = 1",
            HINTS_TABLE, IMAGES_TABLE, TABLE
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
            let hint = PanoramaHint::from_attributes(&record)?;
            let lat = record.require_f64("lat")?;
            let lng = record.require_f64("lng")?;
            let heading = record.require_f64("heading")?;
            res.push((hint, lat, lng, heading));
        }

        Ok(res)
    }

    pub async fn find_hints_by_image_id(&self, image_id: u64) -> Result<Vec<PanoramaHint>> {
        let query =
            SelectQuery::new(HINTS_TABLE).with_condition("image_id", Value::from(image_id as i64));
        let records = self.db.get_records(query).await?;
        records.iter().map(PanoramaHint::from_attributes).collect()
    }

    pub async fn add_hint(&self, hint: &PanoramaHint) -> Result<()> {
        let query = InsertQuery::new(HINTS_TABLE).with_values(hint.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn delete_hints_by_image_id(&self, image_id: u64) -> Result<()> {
        let query =
            DeleteQuery::new(HINTS_TABLE).with_condition("image_id", Value::from(image_id as i64));
        self.db.delete(query).await?;
        Ok(())
    }

    pub async fn transact(&self) -> Result<Self> {
        let db = Arc::new(self.db.transact().await?);
        Ok(Self { db })
    }

    pub async fn commit(&self) -> Result<()> {
        self.db.commit().await
    }
}

impl Injectable for PanoramaRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
