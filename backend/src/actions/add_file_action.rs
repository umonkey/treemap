//! Receive a binary file from the user, attach to a tree.

use crate::services::AppState;
use crate::types::*;
use crate::utils::*;
use actix_web::{post, web::Bytes, web::Data, web::Json, web::Path, HttpRequest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[post("/v1/trees/{id}/files")]
pub async fn add_file_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
    body: Bytes,
) -> Result<Json<FileUploadResponse>> {
    let user_id = state.get_user_id(&req)?;

    let rec = state
        .add_file_handler
        .handle(AddFileRequest {
            user_id,
            tree_id: path.id,
            remote_addr: get_remote_addr(&req).ok_or(Error::RemoteAddrNotSet)?,
            user_agent: get_user_agent(&req).ok_or(Error::UserAgentNotSet)?,
            file: body.to_vec(),
        })
        .await?;

    Ok(Json(rec))
}
