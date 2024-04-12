use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

use crate::services::app::AppState;
use crate::types::TreeDetails;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/trees/{id}")]
pub async fn get_tree(state: Data<AppState>, path: Path<PathInfo>) -> Result<Json<TreeDetails>> {
    let tree = state.get_tree(path.id).await?;
    Ok(Json(tree))
}
