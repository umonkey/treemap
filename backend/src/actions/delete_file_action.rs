use crate::services::AppState;
use crate::types::*;
use actix_web::{delete, web::Data, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[delete("/v1/files/{id}")]
pub async fn delete_file_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .delete_file_handler
        .handle(DeleteFileRequest {
            user_id,
            file_id: path.id,
        })
        .await?;

    Ok(HttpResponse::Accepted().finish())
}
