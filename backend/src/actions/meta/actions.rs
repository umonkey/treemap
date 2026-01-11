//! This handles the tree page request by preparing some HTML metadata
//! and injecting it into the default index.html file.

use crate::services::AppState;
use crate::types::*;
use actix_web::http::header::{CacheControl, CacheDirective, Expires};
use actix_web::web::{Data, Path, ServiceConfig};
use actix_web::{get, HttpResponse};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/{id:\\d+}")]
pub async fn tree_page_action(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    let tree = state.trees.get_tree(path.id).await?;

    let html = state.meta.get_tree(&tree).await?;

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

// Configure the router.
pub fn meta_router(cfg: &mut ServiceConfig) {
    cfg.service(tree_page_action);
}
