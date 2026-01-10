use crate::services::AppState;
use crate::types::*;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{get, HttpRequest};

#[get("")]
pub async fn get_me_action(state: Data<AppState>, req: HttpRequest) -> Result<Json<MeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = state.get_me_handler.handle(user_id).await?;

    Ok(Json(response))
}

#[get("/likes")]
pub async fn get_me_likes_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<LikeList>> {
    let user_id = state.get_user_id(&req)?;
    let res = state.get_me_likes_handler.handle(user_id).await?;
    Ok(Json(res))
}

// Configure the router.
pub fn me_router(cfg: &mut ServiceConfig) {
    cfg.service(get_me_action).service(get_me_likes_action);
}
