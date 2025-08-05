use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/users/{id}/heatmap")]
pub async fn get_user_heatmap_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<Vec<HeatmapResponse>>> {
    let stats = state.get_heatmap_handler.handle_user(path.id).await?;
    Ok(Json(stats))
}
