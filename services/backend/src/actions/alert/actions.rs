use super::schemas::{AddAlertPayload, AlertRead, UpdateAlertPayload};
use crate::domain::alert::{AlertRepository, AlertService};
use crate::domain::alert_photo::AlertPhotoRepository;
use crate::domain::iam::IamService;
use crate::services::alert_loader::{AlertFull, AlertList, AlertLoader};
use crate::services::app::{AlertCreate, AlertManage, OptionalUserId, RequirePermission, UserId};
use crate::services::{AppState, Injected};
use crate::types::{Error, Result};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, patch, post, HttpResponse};
use serde::Deserialize;

/// Maximum number of alerts returned by the admin review list.
const RECENT_ALERTS_LIMIT: u64 = 100;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[post("")]
pub async fn add_alert_action(
    user_id: RequirePermission<AlertCreate>,
    payload: Json<AddAlertPayload>,
    service: Injected<AlertService>,
) -> Result<Json<AlertRead>> {
    let alert = service
        .create(
            *user_id,
            payload.lat,
            payload.lon,
            payload.description.clone(),
            payload.files.clone(),
        )
        .await?;

    // The permission extractor already proved the creator holds `alert:create`.
    Ok(Json(AlertRead::from_domain(&alert, true)))
}

#[get("")]
pub async fn list_alerts_action(
    _user: RequirePermission<AlertManage>,
    alerts: Injected<AlertRepository>,
    loader: Injected<AlertLoader>,
) -> Result<Json<AlertList>> {
    let recent = alerts.get_recent(RECENT_ALERTS_LIMIT).await?;

    Ok(Json(loader.load(&recent).await?))
}

#[get("/{id:\\d+}/full")]
pub async fn get_alert_full_action(
    _user: RequirePermission<AlertManage>,
    path: Path<PathInfo>,
    alerts: Injected<AlertRepository>,
    loader: Injected<AlertLoader>,
) -> Result<Json<AlertFull>> {
    let alert = alerts.get(path.id).await?.ok_or(Error::AlertNotFound)?;

    Ok(Json(loader.load_one(&alert).await?))
}

#[get("/active")]
pub async fn get_active_alerts_action(alerts: Injected<AlertRepository>) -> Result<HttpResponse> {
    let active_alerts = alerts.get_active(7).await?;
    Ok(crate::responders::geo_json::respond_with_alerts(
        &active_alerts,
        7,
    ))
}

#[get("/{id:\\d+}")]
pub async fn get_alert_action(
    user_id: OptionalUserId,
    path: Path<PathInfo>,
    alerts: Injected<AlertRepository>,
    iam: Injected<IamService>,
) -> Result<Json<AlertRead>> {
    let alert = alerts.get(path.id).await?.ok_or(Error::AlertNotFound)?;

    let editable = if *user_id == 0 {
        false
    } else {
        let rights = iam.get_user_rights(*user_id).await?;
        alert.is_editable(*user_id, &rights)
    };

    Ok(Json(AlertRead::from_domain(&alert, editable)))
}

#[patch("/{id:\\d+}")]
pub async fn update_alert_action(
    user_id: UserId,
    path: Path<PathInfo>,
    payload: Json<UpdateAlertPayload>,
    service: Injected<AlertService>,
) -> Result<Json<AlertRead>> {
    let alert = service
        .update_description(path.id, *user_id, &payload.description)
        .await?;

    Ok(Json(AlertRead::from_domain(&alert, true)))
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
