use crate::services::AppState;
use crate::types::*;
use actix_web::get;
use actix_web::web::{Data, Json, ServiceConfig};

#[get("")]
pub async fn get_heatmap_action(state: Data<AppState>) -> Result<Json<Vec<HeatmapResponse>>> {
    let stats = state.get_heatmap_handler.handle().await?;
    Ok(Json(stats))
}

// Configure the router.
pub fn heatmap_router(cfg: &mut ServiceConfig) {
    cfg.service(get_heatmap_action);
}
