use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json};

#[get("/v1/stats/species")]
pub async fn get_species_stats_action(
    state: Data<AppState>,
) -> Result<Json<Vec<SpeciesStatsResponse>>> {
    let res = state.get_species_stats_handler.handle().await?;
    Ok(Json(res))
}
