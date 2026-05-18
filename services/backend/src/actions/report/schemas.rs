use crate::domain::report::Report;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ReportRead {
    pub id: u64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: u64,
    pub description: Option<String>,
}

impl ReportRead {
    pub fn from_domain(report: &Report) -> Self {
        Self {
            id: report.id,
            lat: report.lat,
            lon: report.lon,
            created_at: report.created_at,
            description: report.description.clone(),
        }
    }
}
