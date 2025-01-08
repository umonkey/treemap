use crate::services::AppState;
use crate::types::Result;
use actix_web::{post, web::Data, web::Form, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub code: String,
}

#[post("/v3/login/osm")]
pub async fn login_osm_action(
    state: Data<AppState>,
    payload: Form<Payload>,
) -> Result<HttpResponse> {
    state.login_osm_handler.handle(payload.code.clone()).await?;

    Ok(HttpResponse::Ok().finish())
}
