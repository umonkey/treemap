use super::models::WaterSource;
use super::schemas::*;
use crate::domain::water::{WaterRepository, WaterStatus};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use std::sync::Arc;

pub struct WaterService {
    sources: Arc<WaterRepository>,
}

impl WaterService {
    pub async fn add_source(&self, req: AddWaterRequest) -> Result<WaterSource> {
        let source = WaterSource {
            id: get_unique_id()?,
            created_at: get_timestamp(),
            created_by: req.user_id,
            updated_at: get_timestamp(),
            lat: req.lat,
            lon: req.lon,
            status: WaterStatus::Operational,
        };

        self.sources.add(&source).await?;

        Ok(source)
    }

    pub async fn update_source(&self, req: UpdateWaterRequest) -> Result<WaterSource> {
        let old = self
            .sources
            .get(req.id)
            .await?
            .ok_or(Error::WaterNotFound)?;

        let source = WaterSource {
            id: old.id,
            created_at: old.created_at,
            created_by: old.created_by,
            updated_at: get_timestamp(),
            lat: req.lat,
            lon: req.lon,
            status: old.status,
        };

        self.sources.update(&source).await?;

        Ok(source)
    }

    pub async fn get_source(&self, id: u64) -> Result<WaterSource> {
        self.sources.get(id).await?.ok_or(Error::WaterNotFound)
    }

    pub async fn get_sources(&self, request: &GetWaterRequest) -> Result<Vec<WaterSource>> {
        let mut sources = self.sources.get_by_bounds(request.into()).await?;

        sources.retain(|s| s.status != WaterStatus::Gone);

        Ok(sources)
    }
}

impl Injectable for WaterService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            sources: Arc::new(ctx.build::<WaterRepository>()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;

    async fn setup() -> Arc<WaterService> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<WaterService>()
                .expect("Error creating WaterService"),
        )
    }

    #[tokio::test]
    async fn test_add_and_get_by_bounds() {
        let service = setup().await;

        service
            .add_source(AddWaterRequest {
                lat: 40.0,
                lon: 44.0,
                user_id: 1,
            })
            .await
            .expect("Error adding water source.");

        let sources = service
            .get_sources(&GetWaterRequest {
                n: 45.0,
                e: 45.0,
                s: 35.0,
                w: 35.0,
            })
            .await
            .expect("Error getting water sources.");

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].status, WaterStatus::Operational);
    }

    #[tokio::test]
    async fn test_gone_filtering() {
        let service = setup().await;

        service
            .add_source(AddWaterRequest {
                lat: 40.0,
                lon: 44.0,
                user_id: 1,
            })
            .await
            .expect("Error adding water source.");

        service
            .sources
            .add(&WaterSource {
                id: get_unique_id().expect("Error generating id."),
                created_at: get_timestamp(),
                created_by: 1,
                updated_at: get_timestamp(),
                lat: 40.1,
                lon: 44.1,
                status: WaterStatus::Gone,
            })
            .await
            .expect("Error adding gone water source.");

        let sources = service
            .get_sources(&GetWaterRequest {
                n: 45.0,
                e: 45.0,
                s: 35.0,
                w: 35.0,
            })
            .await
            .expect("Error getting water sources.");

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].status, WaterStatus::Operational);
    }
}
