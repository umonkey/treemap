use crate::types::*;
use actix_web::http::header::{CacheControl, CacheDirective, Expires};
use actix_web::HttpResponse;
use log::error;
use std::time::{Duration, SystemTime};
use tokio::fs;

pub async fn default_action() -> Result<HttpResponse> {
    let path = "static/index.html";

    let body = fs::read(path).await.map_err(|e| {
        error!("Error reading file: {:?}", e);
        Error::FileNotFound
    })?;

    let cache_control = CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(60)]);

    let expiration = SystemTime::now() + Duration::from_secs(60);
    let expiration_header = Expires(expiration.into());

    let res = HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(cache_control)
        .insert_header(expiration_header)
        .body(body);

    Ok(res)
}
