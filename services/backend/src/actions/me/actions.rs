use super::schemas::MeResponse;
use crate::domain::iam::IamService;
use crate::domain::like::LikeService;
use crate::domain::user::UserService;
use crate::services::app::UserId;
use crate::services::like_loader::{LikeList, LikeLoader};
use crate::services::Injected;
use crate::types::*;
use actix_web::get;
use actix_web::web::{Json, ServiceConfig};

#[get("")]
pub async fn get_me_action(
    user_id: UserId,
    user_service: Injected<UserService>,
    iam_service: Injected<IamService>,
) -> Result<Json<MeResponse>> {
    let user = user_service.get_user(*user_id).await?;
    let rights = iam_service.get_user_rights(*user_id).await?;

    Ok(Json(MeResponse::from_domain(user, rights)))
}

#[get("/likes")]
pub async fn get_me_likes_action(
    user_id: UserId,
    like_service: Injected<LikeService>,
    like_loader: Injected<LikeLoader>,
) -> Result<Json<LikeList>> {
    let likes = like_service.get_user_likes(*user_id).await?;
    let res = like_loader.load_list(&likes).await?;
    Ok(Json(res))
}

// Configure the router.
pub fn me_router(cfg: &mut ServiceConfig) {
    cfg.service(get_me_action).service(get_me_likes_action);
}
