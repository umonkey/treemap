/**
 * Receive a binary file from the user, attach to a tree.
 */
use actix_web::{post, web::Bytes, web::Data, web::Json, web::Path, HttpRequest};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{AddFileRequest, FileUploadResponse, Result};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[post("/v1/trees/{id}/files")]
pub async fn add_file(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
    body: Bytes,
) -> Result<Json<FileUploadResponse>> {
    let user_id = state.get_user_id(&req)?;

    let req = AddFileRequest {
        user_id,
        tree_id: path.id,
        file: body.to_vec(),
    };

    let rec = state.add_file(req).await?;

    Ok(Json(rec))
}
