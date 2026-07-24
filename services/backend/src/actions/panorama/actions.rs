use super::schemas::{
    CompleteMultipartRequest, MultipartUploadResponse, PanoramaRead, StartMultipartRequest,
    TrackPoint, UploadUrlResponse, WebVideoUrlResponse,
};
use crate::domain::panorama::{CreatePanorama, PanoramaService, UpdatePanorama};
use crate::services::app::{PanoEdit, RequirePermission};
use crate::services::Injected;
use crate::types::*;
use actix_web::web::{Json, Path};
use actix_web::{get, patch, post, HttpResponse};

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
) -> Result<HttpResponse> {
    let panorama = service.create_panorama(body.into_inner(), *user_id).await?;
    Ok(HttpResponse::Created().json(PanoramaRead::from(panorama)))
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

#[post("/{id}/video")]
pub async fn verify_video_upload_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
) -> Result<Json<PanoramaRead>> {
    let id = path.into_inner();
    let panorama = service.verify_video_upload(id).await?;
    Ok(Json(panorama.into()))
}

#[post("/{id}/video/multipart")]
pub async fn start_video_multipart_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
    body: Json<StartMultipartRequest>,
) -> Result<Json<MultipartUploadResponse>> {
    let id = path.into_inner();
    let (upload_id, urls) = service.start_video_multipart(id, body.parts_count).await?;
    Ok(Json(MultipartUploadResponse { upload_id, urls }))
}

#[post("/{id}/video/multipart/complete")]
pub async fn complete_video_multipart_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
    body: Json<CompleteMultipartRequest>,
) -> Result<Json<PanoramaRead>> {
    let id = path.into_inner();
    let body = body.into_inner();
    let panorama = service
        .complete_video_multipart(id, &body.upload_id, body.parts)
        .await?;
    Ok(Json(panorama.into()))
}

#[get("/{id}/track")]
pub async fn get_track_upload_url_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
) -> Result<Json<UploadUrlResponse>> {
    let id = path.into_inner();
    let url = service.get_track_upload_url(id).await?;
    Ok(Json(UploadUrlResponse { url }))
}

#[get("/{id}/web-video")]
pub async fn get_web_video_url_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
) -> Result<Json<WebVideoUrlResponse>> {
    let id = path.into_inner();
    let url = service.get_web_video_url(id).await?;
    Ok(Json(WebVideoUrlResponse { url }))
}

#[post("/{id}/track")]
pub async fn verify_track_upload_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<PanoramaService>,
    path: Path<u64>,
) -> Result<Json<PanoramaRead>> {
    let id = path.into_inner();
    let panorama = service.verify_track_upload(id).await?;
    Ok(Json(panorama.into()))
}

#[get("/{id}/track.json")]
pub async fn get_panorama_track_action(
    service: Injected<PanoramaService>,
    path: Path<u64>,
) -> Result<Json<Vec<TrackPoint>>> {
    let id = path.into_inner();
    let data = service.get_track_data(id).await?;
    let points = crate::utils::parse_gpx(&data)?;

    let parse_timestamp = |time_str: &str| -> Option<f64> {
        chrono::DateTime::parse_from_rfc3339(time_str)
            .ok()
            .map(|dt| dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0)
    };

    let first_timestamp = points
        .iter()
        .find_map(|p| p.time.as_deref().and_then(parse_timestamp))
        .unwrap_or(0.0);

    let mut track_points = Vec::new();

    for p in points {
        let timestamp = p.time.unwrap_or_default();
        let point_timestamp = if timestamp.is_empty() {
            0.0
        } else {
            parse_timestamp(&timestamp).unwrap_or(0.0)
        };
        let offset = point_timestamp - first_timestamp;
        track_points.push(TrackPoint {
            lat: p.lat,
            lng: p.lon,
            offset,
            timestamp,
        });
    }

    Ok(Json(track_points))
}
