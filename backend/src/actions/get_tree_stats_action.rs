use actix_web::{get, web::Data, web::Json};

use crate::services::AppState;
use crate::types::*;

#[get("/v1/trees/stats")]
pub async fn get_tree_stats_action(state: Data<AppState>) -> Result<Json<TreeStatsResponse>> {
    let stats = state.get_tree_stats_handler.handle().await?;
    Ok(Json(stats))
}
