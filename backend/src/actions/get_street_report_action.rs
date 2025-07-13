use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json, web::Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub address: String,
}

#[get("/v1/streets/report")]
pub async fn get_street_report_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<StreetReport>> {
    let report = state
        .get_street_report_handler
        .handle(&query.address)
        .await?;
    Ok(Json(report))
}
