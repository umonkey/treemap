use super::schemas::PanoramaRead;
use crate::domain::panorama::{CreatePanorama, PanoramaService, UpdatePanorama};
use crate::services::app::{PanoEdit, RequirePermission};
use crate::services::Injected;
use crate::types::*;
use actix_web::web::{Json, Path};
use actix_web::{get, patch, post};

#[get("")]
pub async fn list_panoramas_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
) -> Result<Json<Vec<PanoramaRead>>> {
    let panoramas = service.get_all_panoramas().await?;
    let res = panoramas.into_iter().map(PanoramaRead::from).collect();
    Ok(Json(res))
}

#[post("")]
pub async fn create_panorama_action(
    user_id: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    body: Json<CreatePanorama>,
) -> Result<Json<PanoramaRead>> {
    let panorama = service.create_panorama(body.into_inner(), *user_id).await?;
    Ok(Json(panorama.into()))
}

#[get("/{id}")]
pub async fn get_panorama_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
) -> Result<Json<PanoramaRead>> {
    let id = path.into_inner();
    let panorama = service.get_panorama(id).await?;
    Ok(Json(panorama.into()))
}

#[patch("/{id}")]
pub async fn update_panorama_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
    body: Json<UpdatePanorama>,
) -> Result<Json<PanoramaRead>> {
    let id = path.into_inner();
    let panorama = service.update_panorama(id, body.into_inner()).await?;
    Ok(Json(panorama.into()))
}
