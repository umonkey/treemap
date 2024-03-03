use actix_web::{get, web::Data, web::Path, HttpResponse};
use serde::Deserialize;

use crate::Result;
use crate::services::app::AppState;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/trees/{id}")]
pub async fn get_tree(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<HttpResponse> {
    state.get_tree(path.id).await?;

    Ok(HttpResponse::Accepted()
       .content_type("application/json")
       .body("")
    )
}
