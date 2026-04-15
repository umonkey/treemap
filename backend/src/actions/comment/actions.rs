use crate::domain::comment::CommentService;
use crate::services::comment_loader::{CommentList, CommentLoader};
use crate::services::{AppState, ContextExt};
use crate::types::Result;
use actix_web::get;
use actix_web::web::{Data, Json, ServiceConfig};

#[get("")]
pub async fn get_new_comments_action(state: Data<AppState>) -> Result<Json<CommentList>> {
    let comments = state.build::<CommentService>()?.get_new(100).await?;
    let res = state.build::<CommentLoader>()?.load(&comments).await?;
    Ok(Json(res))
}

// Configure the router.
pub fn comment_router(cfg: &mut ServiceConfig) {
    cfg.service(get_new_comments_action);
}
