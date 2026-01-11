use crate::domain::settings::UpdateSettingsRequest;
use crate::services::AppState;
use crate::types::Result;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{put, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub name: String,
    pub picture: Option<String>,
}

#[put("")]
pub async fn update_settings_action(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .settings
        .update(UpdateSettingsRequest {
            user_id,
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
