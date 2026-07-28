use crate::domain::mapillary::{
    MapillarySequenceDetail, MapillarySequenceSummary, MapillaryService, UpdateMapillarySequence,
};
use crate::services::app::{PanoEdit, RequirePermission};
use crate::services::*;
use crate::types::*;
use actix_web::web::{Json, Path};
use actix_web::{get, patch, HttpResponse};

#[get("/images/{id}")]
pub async fn get_mapillary_image_action(
    service: Injected<MapillaryService>,
    path: Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let image = service.get_image_metadata(&id).await?;

    Ok(HttpResponse::Ok().json(image))
}

#[get("/sequences")]
pub async fn get_mapillary_sequences_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
) -> Result<Json<Vec<MapillarySequenceSummary>>> {
    let sequences = service.get_all_sequences().await?;
    Ok(Json(sequences))
}

#[get("/sequences/{id}")]
pub async fn get_mapillary_sequence_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
) -> Result<Json<MapillarySequenceDetail>> {
    let id = path.into_inner();
    let sequence = service.get_sequence_detail(&id).await?;
    Ok(Json(sequence))
}

#[patch("/sequences/{id}")]
pub async fn update_mapillary_sequence_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
    body: Json<UpdateMapillarySequence>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    service.update_sequence(&id, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
