use crate::domain::training::TrainingService;
use crate::services::app::UserId;
use crate::services::Injected;
use crate::types::*;
use actix_web::{post, web::Json, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub result: f64,
}

#[post("")]
pub async fn add_training_action(
    user_id: UserId,
    training_service: Injected<TrainingService>,
    payload: Json<RequestPayload>,
) -> Result<HttpResponse> {
    training_service.add(*user_id, payload.result).await?;

    Ok(HttpResponse::Accepted().finish())
}
