use crate::services::AppState;
use crate::types::*;
use actix_web::{post, web::Data, web::Json, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub result: f64,
}

#[post("/v1/training")]
pub async fn add_training_action(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .add_training_handler
        .handle(AddTrainingRequest {
            user_id,
            result: payload.result,
        })
        .await?;

    Ok(HttpResponse::Accepted().finish())
}
