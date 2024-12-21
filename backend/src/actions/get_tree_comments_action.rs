use crate::services::AppState;
use crate::types::{CommentList, Result};
use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/trees/{id}/comments")]
pub async fn get_tree_comments_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<CommentList>> {
    let comments = state.get_tree_comments_handler.handle(path.id).await?;
    Ok(Json(comments))
}
