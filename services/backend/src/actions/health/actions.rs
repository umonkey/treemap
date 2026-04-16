use crate::domain::health::HealthService;
use crate::services::{AppState, ContextExt};
use crate::types::Result;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn get_health_action(state: Data<AppState>) -> Result<HttpResponse> {
    state.build::<HealthService>()?.check().await?;
    Ok(HttpResponse::NoContent().finish())
}

// Configure the router.
pub fn health_router(cfg: &mut ServiceConfig) {
    cfg.service(get_health_action);
}
