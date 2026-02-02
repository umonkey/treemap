use actix_web::http::header::{CacheControl, CacheDirective, ETag, EntityTag, Expires};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

use super::schemas::FileStatusResponse;
use crate::services::AppState;
use crate::types::Result;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

async fn get_file_real(state: Data<AppState>, id: u64) -> Result<HttpResponse> {
    let file = state.tree_images.get_file(id).await?;

    let etag = ETag(EntityTag::new_strong(id.to_string()));

    let cache_control = CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(31536000),
    ]);

    let expiration = SystemTime::now() + Duration::from_secs(31536000); // 1 year
    let expiration_header = Expires(expiration.into());

    let res = HttpResponse::Ok()
        .content_type("image/jpeg")
        .insert_header(cache_control)
        .insert_header(etag)
        .insert_header(expiration_header)
        .body(file);

    Ok(res)
}

#[get("/{id:\\d+}.jpg")]
pub async fn get_file_jpg(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    get_file_real(state, path.id).await
}

#[get("/{id:\\d+}")]
pub async fn get_file(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    get_file_real(state, path.id).await
}

#[get("/{id:\\d+}/status")]
pub async fn get_file_status_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<FileStatusResponse>> {
    let status = state.tree_images.get_file_status(path.id).await?;
    Ok(Json(status.into()))
}

#[delete("/{id:\\d+}")]
pub async fn delete_file_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state.tree_images.delete_file(user_id, path.id).await?;

    Ok(HttpResponse::Accepted().finish())
}

// Configure the router.
pub fn file_router(cfg: &mut ServiceConfig) {
    cfg.service(get_file)
        .service(get_file_jpg)
        .service(get_file_status_action)
        .service(delete_file_action);
}
