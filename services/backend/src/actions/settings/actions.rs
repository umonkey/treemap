use crate::domain::settings::{
    AppSettingsService, Settings, SettingsService, UpdateSettingsRequest,
};
use crate::services::app::UserId;
use crate::services::Injected;
use crate::types::Result;
use crate::utils::get_referer_domain;
use actix_web::web::Json;
use actix_web::{get, patch, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[patch("")]
pub async fn update_settings_action(
    user_id: UserId,
    settings_service: Injected<SettingsService>,
    payload: Json<RequestPayload>,
) -> Result<HttpResponse> {
    settings_service
        .update(UpdateSettingsRequest {
            user_id: *user_id,
            name: payload.name.clone(),
            picture: payload.picture.clone(),
        })
        .await?;

    Ok(HttpResponse::Accepted().finish())
}

#[get("")]
pub async fn get_settings_action(
    req: HttpRequest,
    settings_service: Injected<AppSettingsService>,
) -> Result<Json<Settings>> {
    let domain = get_referer_domain(&req)?;

    Ok(Json(settings_service.get(&domain).await?))
}
