use super::schemas::MeResponse;
use crate::domain::like::LikeService;
use crate::domain::user::UserService;
use crate::services::like_loader::{LikeList, LikeLoader};
use crate::services::AppState;
use crate::services::Injected;
use crate::types::*;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{get, HttpRequest};

#[get("")]
pub async fn get_me_action(
    state: Data<AppState>,
    user_service: Injected<UserService>,
    req: HttpRequest,
) -> Result<Json<MeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = user_service.get_user(user_id).await?;

    Ok(Json(response.into()))
}

#[get("/likes")]
pub async fn get_me_likes_action(
    state: Data<AppState>,
    like_service: Injected<LikeService>,
    like_loader: Injected<LikeLoader>,
    req: HttpRequest,
) -> Result<Json<LikeList>> {
    let user_id = state.get_user_id(&req)?;
    let likes = like_service.get_user_likes(user_id).await?;
    let res = like_loader.load_list(&likes).await?;
    Ok(Json(res))
}

// Configure the router.
pub fn me_router(cfg: &mut ServiceConfig) {
    cfg.service(get_me_action).service(get_me_likes_action);
}
