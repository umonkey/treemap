use super::models::{Panorama, PanoramaHint, PanoramaImage};
use crate::domain::tree::Bounds;
use crate::infra::database::{
    Attributes, Database, DeleteQuery, InsertQuery, SelectQuery, UpdateQuery, Value,
};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
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

    #[allow(clippy::too_many_arguments)]
    pub async fn update_panorama_stats_fields(
        &self,
        id: u64,
        min_lat: Option<f64>,
        max_lat: Option<f64>,
        min_lon: Option<f64>,
        max_lon: Option<f64>,
        points_json: Option<String>,
        image_count: i32,
        distance: f64,
    ) -> Result<()> {
        let mut values = Attributes::default();
        values.insert("min_lat", Value::from(min_lat));
        values.insert("max_lat", Value::from(max_lat));
        values.insert("min_lon", Value::from(min_lon));
        values.insert("max_lon", Value::from(max_lon));
        values.insert("points_json", Value::from(points_json));
        values.insert("image_count", Value::from(image_count as i64));
        values.insert("distance", Value::from(distance));

        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_values(values);
        self.db.update(query).await?;
        Ok(())
    }

    pub async fn delete_images(&self, panorama_id: u64) -> Result<u64> {
        let query = DeleteQuery::new(IMAGES_TABLE)
            .with_condition("panorama_id", Value::from(panorama_id as i64));
        self.db.delete(query).await
    }

    pub async fn count_hints_by_panorama_id(&self, panorama_id: u64) -> Result<u64> {
        let sql = format!(
            "SELECT COUNT(1) AS cnt FROM `{}` WHERE `image_id` IN (SELECT `id` FROM `{}` WHERE `panorama_id` = ?)",
            HINTS_TABLE, IMAGES_TABLE
        );
        let params = &[Value::from(panorama_id as i64)];
        let records = self.db.fetch_sql(&sql, params).await?;
        if let Some(record) = records.first() {
            record.require_u64("cnt")
        } else {
            Ok(0)
        }
    }

    pub async fn delete_hints_by_panorama_id(&self, panorama_id: u64) -> Result<u64> {
        let sql = format!(
            "DELETE FROM `{}` WHERE `image_id` IN (SELECT `id` FROM `{}` WHERE `panorama_id` = ?)",
            HINTS_TABLE, IMAGES_TABLE
        );
        let params = &[Value::from(panorama_id as i64)];
        self.db.execute_sql(&sql, params).await
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
            .with_condition("panorama_id", Value::from(panorama_id as i64))
            .with_order("id");
        let records = self.db.get_records(query).await?;
        records.iter().map(PanoramaImage::from_attributes).collect()
    }

    pub async fn update_image(&self, image: &PanoramaImage) -> Result<()> {
        let query = UpdateQuery::new(IMAGES_TABLE)
            .with_condition("id", Value::from(image.id as i64))
            .with_values(image.to_attributes());
        self.db.update(query).await?;
        Ok(())
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

    pub async fn find_images_by_bounds(
        &self,
        bounds: Bounds,
    ) -> Result<Vec<(PanoramaImage, i64, f64, f64)>> {
        let sql = format!(
            "SELECT i.*, p.created_at, p.lat_offset, p.lon_offset FROM `{}` i INNER JOIN `{}` p ON i.panorama_id = p.id WHERE i.`lat` + p.lat_offset <= ? AND i.lat + p.lat_offset >= ? AND i.lng + p.lon_offset <= ? AND i.lng + p.lon_offset >= ? AND i.hidden = 0 AND p.status = 'SUCCESS' AND p.visible = 1",
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
            let lat_offset = record.get_f64("lat_offset")?.unwrap_or(0.0);
            let lon_offset = record.get_f64("lon_offset")?.unwrap_or(0.0);
            res.push((img, created_at, lat_offset, lon_offset));
        }

        Ok(res)
    }

    pub async fn find_hints_with_location_by_bounds(
        &self,
        bounds: Bounds,
    ) -> Result<Vec<(PanoramaHint, f64, f64, f64, f64, f64)>> {
        let sql = format!(
            "SELECT h.*, i.lat, i.lng, i.heading, p.lat_offset, p.lon_offset FROM `{}` h INNER JOIN `{}` i ON h.image_id = i.id INNER JOIN `{}` p ON i.panorama_id = p.id WHERE i.`lat` + p.lat_offset <= ? AND i.lat + p.lat_offset >= ? AND i.lng + p.lon_offset <= ? AND i.lng + p.lon_offset >= ? AND i.hidden = 0 AND p.status = 'SUCCESS' AND p.visible = 1",
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
            let lat_offset = record.get_f64("lat_offset")?.unwrap_or(0.0);
            let lon_offset = record.get_f64("lon_offset")?.unwrap_or(0.0);
            res.push((hint, lat, lng, heading, lat_offset, lon_offset));
        }

        Ok(res)
    }

    pub async fn find_hints_by_panorama(
        &self,
        panorama_id: u64,
    ) -> Result<Vec<(PanoramaHint, f64, f64, f64)>> {
        let sql = format!(
            "SELECT h.*, i.lat, i.lng, i.heading FROM `{}` h INNER JOIN `{}` i ON h.image_id = i.id WHERE i.panorama_id = ? AND i.hidden = 0",
            HINTS_TABLE, IMAGES_TABLE
        );

        let params = &[Value::from(panorama_id as i64)];

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
        let mut attrs = hint.to_attributes();
        attrs.insert("created_at", Value::from(get_timestamp() as i64));
        let query = InsertQuery::new(HINTS_TABLE).with_values(attrs);
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn delete_hints_by_image_id(&self, image_id: u64) -> Result<()> {
        let query =
            DeleteQuery::new(HINTS_TABLE).with_condition("image_id", Value::from(image_id as i64));
        self.db.delete(query).await?;
        Ok(())
    }

    pub async fn delete_hints_by_user_since(&self, user_id: u64, since: u64) -> Result<u64> {
        let sql = format!(
            "DELETE FROM `{}` WHERE `user_id` = ? AND `created_at` >= ?",
            HINTS_TABLE
        );
        let params = &[Value::from(user_id as i64), Value::from(since as i64)];
        self.db.execute_sql(&sql, params).await
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;

    async fn setup() -> Arc<PanoramaRepository> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<PanoramaRepository>()
                .expect("Error creating panorama repository."),
        )
    }

    #[tokio::test]
    async fn test_delete_hints_by_user_since() {
        let repo = setup().await;
        let user_id = 987_654_321;
        let now = get_timestamp();

        repo.add_hint(&PanoramaHint {
            image_id: 1,
            angle: 0.0,
            user_id,
        })
        .await
        .expect("Error adding a recent hint.");

        let sql = format!(
            "INSERT INTO `{}` (`image_id`, `angle`, `user_id`, `created_at`) VALUES (?, ?, ?, ?)",
            HINTS_TABLE
        );
        let params = &[
            Value::from(2_i64),
            Value::from(0.0_f64),
            Value::from(user_id as i64),
            Value::from((now - 600) as i64),
        ];
        repo.db
            .execute_sql(&sql, params)
            .await
            .expect("Error adding an old hint.");

        let deleted = repo
            .delete_hints_by_user_since(user_id, now - 300)
            .await
            .expect("Error deleting recent hints.");

        assert_eq!(1, deleted);

        let count_sql = format!(
            "SELECT COUNT(1) AS cnt FROM `{}` WHERE `user_id` = ?",
            HINTS_TABLE
        );
        let count_params = &[Value::from(user_id as i64)];
        let records = repo
            .db
            .fetch_sql(&count_sql, count_params)
            .await
            .expect("Error counting hints.");
        let remaining = records
            .first()
            .expect("Missing count record.")
            .require_u64("cnt")
            .expect("Error reading count.");

        assert_eq!(1, remaining);
    }
}
