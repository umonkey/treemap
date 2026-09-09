use super::schemas::*;
use crate::domain::water::{
    AddWaterRequest, GetWaterRequest, UpdateWaterRequest, WaterService, WaterSource,
};
use crate::services::app::UserId;
use crate::services::Injected;
use crate::types::Result;
use actix_web::web::{Json, Path, Query};
use actix_web::{get, patch, post, HttpResponse};

#[post("")]
pub async fn add_water_action(
    user_id: UserId,
    payload: Json<AddWaterPayload>,
    service: Injected<WaterService>,
) -> Result<Json<WaterSource>> {
    let source = service
        .add_source(AddWaterRequest {
            lat: payload.lat,
            lon: payload.lon,
            user_id: *user_id,
        })
        .await?;

    Ok(Json(source))
}

#[patch("/{id:\\d+}")]
pub async fn update_water_action(
    user_id: UserId,
    path: Path<PathInfo>,
    payload: Json<UpdateWaterPayload>,
    service: Injected<WaterService>,
) -> Result<Json<WaterSource>> {
    let source = service
        .update_source(UpdateWaterRequest {
            id: path.id,
            lat: payload.lat,
            lon: payload.lon,
            user_id: *user_id,
        })
        .await?;

    Ok(Json(source))
}

#[get("/geo.json")]
pub async fn get_water_json_action(
    query: Query<GetWaterRequest>,
    service: Injected<WaterService>,
) -> Result<HttpResponse> {
    let sources = service.get_sources(&query).await?;

    Ok(crate::responders::geo_json::respond_with_water(&sources))
}

#[get("/{id:\\d+}")]
pub async fn get_water_action(
    path: Path<PathInfo>,
    service: Injected<WaterService>,
) -> Result<Json<WaterSource>> {
    let source = service.get_source(path.id).await?;

    Ok(Json(source))
}
