use crate::services::AppState;
use crate::types::*;
use actix_web::web::ServiceConfig;
use actix_web::{post, web::Data, web::Json, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub result: f64,
}

#[post("")]
pub async fn add_training_action(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state.training.add(user_id, payload.result).await?;

    Ok(HttpResponse::Accepted().finish())
}

// Configure the router.
pub fn training_router(cfg: &mut ServiceConfig) {
    cfg.service(add_training_action);
}
