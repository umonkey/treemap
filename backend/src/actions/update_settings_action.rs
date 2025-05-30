use crate::services::AppState;
use crate::types::*;
use actix_web::{put, web::Data, web::Json, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub name: String,
    pub picture: Option<String>,
}

#[put("/v1/settings")]
pub async fn update_settings_action(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .update_settings_handler
        .handle(UpdateSettingsRequest {
            user_id,
            name: payload.name.clone(),
            picture: payload.picture.clone(),
        })
        .await?;

    Ok(HttpResponse::Accepted().finish())
}
