use actix_web::{get, web::Data, web::Json, HttpRequest};

use crate::services::AppState;
use crate::types::{MeResponse, Result};

#[get("/v1/me")]
pub async fn get_me(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<MeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = state.get_user_info(user_id).await?;

    Ok(Json(response))
}
