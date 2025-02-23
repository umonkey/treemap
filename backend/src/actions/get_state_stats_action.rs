use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json};

#[get("/v1/stats/state")]
pub async fn get_state_stats_action(
    state: Data<AppState>,
) -> Result<Json<Vec<StateStatsResponse>>> {
    let stats = state.get_state_stats_handler.handle().await?;
    Ok(Json(stats))
}
