use crate::domain::comment::{CommentEntity, CommentService};
use crate::services::app::{CommentCreate, RequirePermission};
use crate::services::comment_loader::{CommentList, CommentLoader};
use crate::services::Injected;
use crate::types::{Error, Result};
use actix_web::get;
use actix_web::web::{Json, Path};
use actix_web::{post, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommentPath {
    pub entity_type: String,
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct RequestPayload {
    pub message: String,
}

#[get("/{entity_type}/{id:\\d+}")]
pub async fn get_comments_action(
    path: Path<CommentPath>,
    service: Injected<CommentService>,
    loader: Injected<CommentLoader>,
) -> Result<Json<CommentList>> {
    let entity_type = CommentEntity::parse(&path.entity_type).ok_or_else(|| {
        Error::BadRequestMessage(format!("Unknown comment entity type: {}", path.entity_type))
    })?;

    let comments = service.get_comments(entity_type, path.id).await?;
    let res = loader.load(&comments).await?;

    Ok(Json(res))
}

#[post("/{entity_type}/{id:\\d+}")]
pub async fn add_comment_action(
    user_id: RequirePermission<CommentCreate>,
    path: Path<CommentPath>,
    payload: Json<RequestPayload>,
    service: Injected<CommentService>,
) -> Result<HttpResponse> {
    let entity_type = CommentEntity::parse(&path.entity_type).ok_or_else(|| {
        Error::BadRequestMessage(format!("Unknown comment entity type: {}", path.entity_type))
    })?;

    service
        .add_comment(entity_type, path.id, Some(*user_id), &payload.message)
        .await?;

    Ok(HttpResponse::Accepted().finish())
}

#[get("")]
pub async fn get_new_comments_action(
    comments_service: Injected<CommentService>,
    loader: Injected<CommentLoader>,
) -> Result<Json<CommentList>> {
    let comments = comments_service.get_new(100).await?;
    let res = loader.load(&comments).await?;

    Ok(Json(res))
}
