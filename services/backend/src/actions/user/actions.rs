use super::schemas::{UserList, UserReadWithRights};
use crate::domain::heatmap::{HeatmapItem, HeatmapService};
use crate::domain::iam::IamService;
use crate::domain::user::{UserService, UserUpdate};
use crate::services::app::{RequirePermission, UserManage};
use crate::services::Injected;
use crate::types::*;
use actix_web::{get, put, web::Json, web::Path, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("")]
pub async fn get_users(
    _user: RequirePermission<UserManage>,
    user_service: Injected<UserService>,
    iam_service: Injected<IamService>,
) -> Result<Json<UserList>> {
    let users = user_service.list().await?;
    let mut records = Vec::new();

    for user in users {
        let rights = iam_service.get_user_rights(user.id).await?;
        records.push(UserReadWithRights::from_domain(user, rights));
    }

    Ok(Json(UserList { users: records }))
}

#[get("/{id}")]
pub async fn get_user(
    user_service: Injected<UserService>,
    iam_service: Injected<IamService>,
    path: Path<PathInfo>,
) -> Result<Json<UserReadWithRights>> {
    let user = user_service.get_user(path.id).await?;
    let rights = iam_service.get_user_rights(path.id).await?;

    Ok(Json(UserReadWithRights::from_domain(user, rights)))
}

#[get("/{id}/heatmap")]
pub async fn get_user_heatmap(
    heatmap_service: Injected<HeatmapService>,
    path: Path<PathInfo>,
) -> Result<Json<Vec<HeatmapItem>>> {
    let stats = heatmap_service.get_user(path.id).await?;
    Ok(Json(stats))
}

#[put("/{id}")]
pub async fn update_user_action(
    _user: RequirePermission<UserManage>,
    user_service: Injected<UserService>,
    path: Path<PathInfo>,
    body: Json<UserUpdate>,
) -> Result<HttpResponse> {
    user_service.update_user(path.id, body.into_inner()).await?;

    Ok(HttpResponse::Accepted().finish())
}
