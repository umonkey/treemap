//! Receive a binary file from the user, attach to a tree.

use crate::actions::tree::FileUploadResponse;
use crate::services::AppState;
use crate::types::*;
use crate::utils::*;
use actix_web::web::{Bytes, Data, Json, ServiceConfig};
use actix_web::{post, HttpRequest};

#[post("")]
pub async fn upload_action(
    state: Data<AppState>,
    req: HttpRequest,
    body: Bytes,
) -> Result<Json<FileUploadResponse>> {
    let user_id = state.get_user_id(&req)?;

    let rec = state
        .upload_handler
        .handle(FileUploadRequest {
            user_id,
            file: body.to_vec(),
            remote_addr: get_remote_addr(&req).ok_or(Error::RemoteAddrNotSet)?,
            user_agent: get_user_agent(&req).ok_or(Error::UserAgentNotSet)?,
        })
        .await?;

    Ok(Json(rec))
}

// Configure the router.
pub fn upload_router(cfg: &mut ServiceConfig) {
    cfg.service(upload_action);
}
