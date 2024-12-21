use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{FileStatusResponse, Result};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/files/{id}/status")]
pub async fn get_file_status_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<FileStatusResponse>> {
    let status = state.get_file_status_handler.handle(path.id).await?;
    Ok(Json(status))
}
