use crate::services::meta::MetaService;
use crate::services::Injected;
use crate::types::*;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{HttpRequest, HttpResponse};
use log::error;
use tokio::fs;

pub async fn default_action(
    req: HttpRequest,
    meta_service: Injected<MetaService>,
) -> Result<HttpResponse> {
    let path = "static/index.html";

    let body = fs::read_to_string(path).await.map_err(|e| {
        error!("Error reading file: {e:?}");
        Error::FileNotFound
    })?;

    let html = meta_service.inject_robots(&body, req.path());

    let cache_control = CacheControl(vec![
        CacheDirective::NoCache,
        CacheDirective::NoStore,
        CacheDirective::MustRevalidate,
    ]);

    let res = HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(cache_control)
        .body(html);

    Ok(res)
}
