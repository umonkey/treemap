use crate::services::comment_loader::CommentList;
use crate::services::AppState;
use crate::types::Result;
use actix_web::get;
use actix_web::web::{Data, Json, ServiceConfig};

#[get("/v1/comments")]
pub async fn get_new_comments_action(state: Data<AppState>) -> Result<Json<CommentList>> {
    let comments = state.comments.get_new(100).await?;
    let res = state.comment_loader.load(&comments).await?;
    Ok(Json(res))
}

// Configure the router.
pub fn comment_router(cfg: &mut ServiceConfig) {
    cfg.service(get_new_comments_action);
}
