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

    // `no-cache` keeps this response out of stale reuse while still allowing the document to live
    // in the back/forward cache; `no-store` would disable bfcache in Firefox and Safari.
    let cache_control = CacheControl(vec![
        CacheDirective::NoCache,
        CacheDirective::MustRevalidate,
    ]);

    let res = HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(cache_control)
        .body(html);

    Ok(res)
}
