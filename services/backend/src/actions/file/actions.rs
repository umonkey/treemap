use actix_web::{delete, get, web::Json, web::Path, HttpResponse};
use serde::Deserialize;

use super::schemas::FileStatusResponse;
use crate::domain::file::FileService;
use crate::services::app::UserId;
use crate::services::Injected;
use crate::types::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/{id:\\d+}/status")]
pub async fn get_file_status_action(
    file_service: Injected<FileService>,
    path: Path<PathInfo>,
) -> Result<Json<FileStatusResponse>> {
    let file = file_service
        .get(path.id)
        .await?
        .ok_or(Error::FileNotFound)?;

    Ok(Json(file.into()))
}

#[delete("/{id:\\d+}")]
pub async fn delete_file_action(
    user_id: UserId,
    file_service: Injected<FileService>,
    path: Path<PathInfo>,
) -> Result<HttpResponse> {
    file_service.delete(*user_id, path.id).await?;

    Ok(HttpResponse::Accepted().finish())
}
