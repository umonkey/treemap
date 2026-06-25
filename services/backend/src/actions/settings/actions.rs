use crate::domain::settings::{SettingsService, UpdateSettingsRequest};
use crate::services::app::UserId;
use crate::services::Injected;
use crate::types::Result;
use actix_web::web::{Json, ServiceConfig};
use actix_web::{put, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub name: String,
    pub picture: Option<String>,
}

#[put("")]
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

// Configure the router.
pub fn settings_router(cfg: &mut ServiceConfig) {
    cfg.service(update_settings_action);
}
