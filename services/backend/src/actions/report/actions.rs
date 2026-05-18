use super::schemas::ReportRead;
use crate::domain::report::ReportRepository;
use crate::domain::report_photo::ReportPhotoRepository;
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
pub async fn get_active_reports_action(
    reports: Injected<ReportRepository>,
) -> Result<HttpResponse> {
    let active_reports = reports.get_active().await?;
    Ok(crate::responders::geo_json::respond_with_reports(
        &active_reports,
    ))
}

#[get("/{id:\\d+}")]
pub async fn get_report_action(
    path: Path<PathInfo>,
    reports: Injected<ReportRepository>,
) -> Result<Json<ReportRead>> {
    let report = reports
        .get(path.id)
        .await?
        .ok_or(Error::DatabaseStructure("Report not found".to_string()))?;

    Ok(Json(ReportRead::from_domain(&report)))
}

#[get("/{id:\\d+}/photos")]
pub async fn get_report_photos_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    photos: Injected<ReportPhotoRepository>,
) -> Result<Json<Vec<String>>> {
    let report_photos = photos.get_by_report(path.id).await?;
    let base_url = &state.config.files_base_url;

    let urls = report_photos
        .into_iter()
        .map(|p| format!("{}{}", base_url, p.photo_path))
        .collect();

    Ok(Json(urls))
}

pub fn report_router(cfg: &mut ServiceConfig) {
    cfg.service(get_active_reports_action)
        .service(get_report_action)
        .service(get_report_photos_action);
}
