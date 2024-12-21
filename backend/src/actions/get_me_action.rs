use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json, HttpRequest};

#[get("/v1/me")]
pub async fn get_me_action(state: Data<AppState>, req: HttpRequest) -> Result<Json<MeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = state.get_me_handler.handle(user_id).await?;

    Ok(Json(response))
}
