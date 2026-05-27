use crate::types::*;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::HttpResponse;
use log::error;
use tokio::fs;

pub async fn default_action() -> Result<HttpResponse> {
    let path = "static/index.html";

    let body = fs::read(path).await.map_err(|e| {
        error!("Error reading file: {e:?}");
        Error::FileNotFound
    })?;

    let cache_control = CacheControl(vec![
        CacheDirective::NoCache,
        CacheDirective::NoStore,
        CacheDirective::MustRevalidate,
    ]);

    let res = HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(cache_control)
        .body(body);

    Ok(res)
}
