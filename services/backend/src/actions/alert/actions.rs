use super::schemas::AlertRead;
use crate::domain::alert::AlertRepository;
use crate::domain::alert_photo::AlertPhotoRepository;
use crate::services::{AppState, Injected};
use crate::types::{Error, Result};
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/active")]
pub async fn get_active_alerts_action(alerts: Injected<AlertRepository>) -> Result<HttpResponse> {
    let active_alerts = alerts.get_active().await?;
    Ok(crate::responders::geo_json::respond_with_alerts(
        &active_alerts,
    ))
}

#[get("/{id:\\d+}")]
pub async fn get_alert_action(
    path: Path<PathInfo>,
    alerts: Injected<AlertRepository>,
) -> Result<Json<AlertRead>> {
    let alert = alerts
        .get(path.id)
        .await?
        .ok_or(Error::DatabaseStructure("Alert not found".to_string()))?;

    Ok(Json(AlertRead::from_domain(&alert)))
}

#[get("/{id:\\d+}/photos")]
pub async fn get_alert_photos_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    photos: Injected<AlertPhotoRepository>,
) -> Result<Json<Vec<String>>> {
    let alert_photos = photos.get_by_alert(path.id).await?;
    let base_url = &state.config.files_base_url;

    let urls = alert_photos
        .into_iter()
        .map(|p| format!("{}{}", base_url, p.photo_path))
        .collect();

    Ok(Json(urls))
}

pub fn alert_router(cfg: &mut ServiceConfig) {
    cfg.service(get_active_alerts_action)
        .service(get_alert_action)
        .service(get_alert_photos_action);
}
