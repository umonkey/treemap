//! This handles the tree page request by preparing some HTML metadata
//! and injecting it into the default index.html file.

use crate::domain::tree::TreeService;
use crate::services::meta::MetaService;
use crate::services::Injected;
use crate::types::*;
use actix_web::http::header::{CacheControl, CacheDirective, Expires};
use actix_web::web::{Path, ServiceConfig};
use actix_web::{get, HttpResponse};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/{id:\\d+}")]
pub async fn tree_page_action(
    tree_service: Injected<TreeService>,
    meta_service: Injected<MetaService>,
    path: Path<PathInfo>,
) -> Result<HttpResponse> {
    serve_tree_meta(tree_service, meta_service, path.id).await
}

#[get("/{id:\\d+}/preview")]
pub async fn tree_preview_action(
    tree_service: Injected<TreeService>,
    meta_service: Injected<MetaService>,
    path: Path<PathInfo>,
) -> Result<HttpResponse> {
    serve_tree_meta(tree_service, meta_service, path.id).await
}

async fn serve_tree_meta(
    tree_service: Injected<TreeService>,
    meta_service: Injected<MetaService>,
    id: u64,
) -> Result<HttpResponse> {
    let tree = tree_service.get_tree(id).await?;

    let html = meta_service.get_tree(&tree).await?;

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
    cfg.service(tree_preview_action);
}
