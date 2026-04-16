use crate::domain::health::HealthService;
use crate::services::Injected;
use crate::types::Result;
use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn get_health_action(health_service: Injected<HealthService>) -> Result<HttpResponse> {
    health_service.check().await?;
    Ok(HttpResponse::NoContent().finish())
}

// Configure the router.
pub fn health_router(cfg: &mut ServiceConfig) {
    cfg.service(get_health_action);
}
