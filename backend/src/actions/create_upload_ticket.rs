use actix_web::{post, web::Data, web::Json, HttpRequest};

use crate::services::AppState;
use crate::types::{Result, UploadTicket};

#[post("/v1/uploads")]
pub async fn create_upload_ticket(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<UploadTicket>> {
    let user_id = state.get_user_id(&req)?;
    let ticket = state.create_upload_ticket(user_id).await?;

    Ok(Json(ticket))
}
