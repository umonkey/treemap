//! Handle file upload tickets and finalization.

use super::schemas::*;
use crate::actions::tree::FileUploadResponse;
use crate::domain::upload::UploadService;
use crate::services::app::UserId;
use crate::services::Injected;
use crate::types::*;
use crate::utils::*;
use actix_web::web::{Json, Path};
use actix_web::{post, HttpRequest};

#[post("")]
pub async fn upload_action(
    upload_service: Injected<UploadService>,
    user_id: UserId,
    req: HttpRequest,
    payload: Json<UploadTicketRequest>,
) -> Result<Json<FileUploadResponse>> {
    let remote_addr = get_remote_addr(&req).ok_or(Error::RemoteAddrNotSet)?;
    let user_agent = get_user_agent(&req).ok_or(Error::UserAgentNotSet)?;

    let rec = upload_service
        .create_upload_ticket(*user_id, payload.size, remote_addr, user_agent)
        .await?;

    Ok(Json(rec))
}

#[post("/{id}/finish")]
pub async fn finish_upload_action(
    upload_service: Injected<UploadService>,
    path: Path<u64>,
) -> Result<Json<()>> {
    let id = path.into_inner();
    upload_service.finish_upload(id).await?;
    Ok(Json(()))
}
