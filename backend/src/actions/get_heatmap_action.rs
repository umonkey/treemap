use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json};

#[get("/v1/heatmap")]
pub async fn get_heatmap_action(state: Data<AppState>) -> Result<Json<Vec<HeatmapResponse>>> {
    let stats = state.get_heatmap_handler.handle().await?;
    Ok(Json(stats))
}
