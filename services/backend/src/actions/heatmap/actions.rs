use crate::domain::heatmap::{HeatmapItem, HeatmapService};
use crate::services::Injected;
use crate::types::*;
use actix_web::get;
use actix_web::web::Json;

#[get("")]
pub async fn get_heatmap_action(
    heatmap_service: Injected<HeatmapService>,
) -> Result<Json<Vec<HeatmapItem>>> {
    let stats = heatmap_service.get_total().await?;
    Ok(Json(stats))
}
