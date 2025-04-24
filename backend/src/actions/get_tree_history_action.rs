use crate::services::AppState;
use crate::types::{PropList, Result};
use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/trees/{id}/history")]
pub async fn get_tree_history_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<PropList>> {
    let comments = state.get_tree_history_handler.handle(path.id).await?;
    Ok(Json(comments))
}
