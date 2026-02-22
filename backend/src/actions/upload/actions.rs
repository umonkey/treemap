//! Handle file upload tickets and finalization.

use super::schemas::*;
use crate::actions::tree::FileUploadResponse;
use crate::services::AppState;
use crate::types::*;
use crate::utils::*;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{post, HttpRequest};

#[post("")]
pub async fn upload_action(
    state: Data<AppState>,
    req: HttpRequest,
    payload: Json<UploadTicketRequest>,
) -> Result<Json<FileUploadResponse>> {
    let user_id = state.get_user_id(&req)?;

    let remote_addr = get_remote_addr(&req).ok_or(Error::RemoteAddrNotSet)?;
    let user_agent = get_user_agent(&req).ok_or(Error::UserAgentNotSet)?;

    let rec = state
        .uploads
        .create_upload_ticket(user_id, payload.size, remote_addr, user_agent)
        .await?;

    Ok(Json(rec))
}

#[post("/{id}/finish")]
pub async fn finish_upload_action(
    state: Data<AppState>,
    path: Path<u64>,
    payload: Json<FinishUploadRequest>,
) -> Result<Json<()>> {
    let id = path.into_inner();
    state.uploads.finish_upload(id, payload.size).await?;
    Ok(Json(()))
}

// Configure the router.
pub fn upload_router(cfg: &mut ServiceConfig) {
    cfg.service(upload_action);
    cfg.service(finish_upload_action);
}
