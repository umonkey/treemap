use crate::domain::export::models::ExportFile;
use crate::domain::export::ExportService;
use crate::services::Injected;
use crate::types::*;
use actix_web::{get, web::Json};

#[get("/files")]
pub async fn get_export_files_action(
    service: Injected<ExportService>,
) -> Result<Json<Vec<ExportFile>>> {
    Ok(Json(service.list_recent(10).await?))
}
