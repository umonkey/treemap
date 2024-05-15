use actix_web::http::header::{CacheControl, CacheDirective, ETag, EntityTag, Expires};
use actix_web::{get, web::Data, web::Path, HttpResponse};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

use crate::services::AppState;
use crate::types::Result;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

async fn get_file_real(state: Data<AppState>, id: u64) -> Result<HttpResponse> {
    let file = state.get_file(id).await?;

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

#[get("/v1/files/{id}")]
pub async fn get_file(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    get_file_real(state, path.id).await
}

#[get("/v1/files/{id}.jpg")]
pub async fn get_file_jpg(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    get_file_real(state, path.id).await
}
