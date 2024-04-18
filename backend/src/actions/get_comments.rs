use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{PublicCommentInfo, Result};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/trees/{id}/comments")]
pub async fn get_comments(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<Vec<PublicCommentInfo>>> {
    let comments = state.get_comments(path.id).await?;
    Ok(Json(comments))
}
