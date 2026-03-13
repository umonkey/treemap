use super::schemas::StreetRead;
use crate::domain::street::StreetReport;
use crate::responders::csv::trees_to_csv;
use crate::services::AppState;
use crate::types::Result;
use actix_web::web::{Data, Json, Query, ServiceConfig};
use actix_web::{get, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub address: String,
}

#[derive(Deserialize)]
pub struct CsvReportQuery {
    pub address: String,
}

#[get("/search")]
pub async fn search_streets_action(
    state: Data<AppState>,
    query: Query<SearchQuery>,
) -> Result<Json<Vec<StreetRead>>> {
    let records = state.streets.search(&query.query).await?;
    let res = records.iter().map(|f| f.into()).collect();

    Ok(Json(res))
}

#[get("/report")]
pub async fn get_street_report_action(
    state: Data<AppState>,
    query: Query<ReportQuery>,
) -> Result<Json<StreetReport>> {
    let report = state.streets.get_report(&query.address).await?;
    Ok(Json(report))
}

#[get("/report.csv")]
pub async fn get_street_csv_report_action(
    state: Data<AppState>,
    query: Query<CsvReportQuery>,
) -> Result<HttpResponse> {
    let trees = state.streets.get_trees_on_street(&query.address).await?;
    let filename = format!("report-{}", query.address);
    trees_to_csv(trees, &filename)
}

// Configure the router.
pub fn street_router(cfg: &mut ServiceConfig) {
    cfg.service(search_streets_action)
        .service(get_street_report_action)
        .service(get_street_csv_report_action);
}
