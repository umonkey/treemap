//! This handles the tree page request by preparing some HTML metadata
//! and injecting it into the default index.html file.

use crate::services::AppState;
use crate::types::*;
use actix_web::http::header::{CacheControl, CacheDirective, Expires};
use actix_web::HttpResponse;
use actix_web::{get, web::Data, web::Path};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/tree/{id}")]
pub async fn tree_page_action(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    let html = state.tree_page_handler.handle(path.id).await?;

    let cache_control = CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(60)]);

    let expiration = SystemTime::now() + Duration::from_secs(60);
    let expiration_header = Expires(expiration.into());

    let res = HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(cache_control)
        .insert_header(expiration_header)
        .body(html);

    Ok(res)
}
