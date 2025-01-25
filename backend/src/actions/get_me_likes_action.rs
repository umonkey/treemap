use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json, HttpRequest};

#[get("/v1/me/likes")]
pub async fn get_me_likes_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<LikeList>> {
    let user_id = state.get_user_id(&req)?;
    let res = state.get_me_likes_handler.handle(user_id).await?;
    Ok(Json(res))
}
