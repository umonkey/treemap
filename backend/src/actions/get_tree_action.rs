use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/trees/{id}")]
pub async fn get_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<SingleTreeResponse>> {
    let tree = state.get_tree_handler.handle(path.id).await?;
    Ok(Json(tree))
}
