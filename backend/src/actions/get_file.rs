use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{get, web::Data, web::Path, HttpResponse};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::Result;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/files/{id}")]
pub async fn get_file(state: Data<AppState>, path: Path<PathInfo>) -> Result<HttpResponse> {
    let file = state.get_file(path.id).await?;

    let res = HttpResponse::Ok()
        .content_type("image/jpeg")
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(31536000),
        ]))
        .body(file);

    Ok(res)
}
