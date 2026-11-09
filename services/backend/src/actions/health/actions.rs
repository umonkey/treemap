use crate::domain::health::HealthService;
use crate::services::Injected;
use crate::types::Result;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{route, HttpResponse};

#[route("", method = "GET", method = "HEAD")]
pub async fn get_health_action(health_service: Injected<HealthService>) -> Result<HttpResponse> {
    health_service.check().await?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .insert_header(CacheControl(vec![CacheDirective::NoStore]))
        .body("It Works!"))
}
