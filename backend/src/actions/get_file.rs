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

#[get("/v1/files/{id}")]
pub async fn get_file(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    let file = state.get_file(path.id).await?;

    let etag = ETag(EntityTag::new_strong(path.id.to_string()));

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
