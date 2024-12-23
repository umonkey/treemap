use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json, web::Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub count: Option<u64>,
    pub skip: Option<u64>,
}

#[get("/v1/stats/species/mismatch")]
pub async fn get_species_mismatch_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<TreeList>> {
    let count = query.count.unwrap_or(100);
    let skip = query.skip.unwrap_or(0);

    let res = state
        .get_species_mismatch_handler
        .handle(count, skip)
        .await?;
    Ok(Json(res))
}
