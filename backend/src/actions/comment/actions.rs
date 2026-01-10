use crate::services::AppState;
use crate::types::{CommentList, Result};
use actix_web::get;
use actix_web::web::{Data, Json, ServiceConfig};

#[get("/v1/comments")]
pub async fn get_new_comments_action(state: Data<AppState>) -> Result<Json<CommentList>> {
    let res = state.get_new_comments_handler.handle(100).await?;
    Ok(Json(res))
}

// Configure the router.
pub fn comment_router(cfg: &mut ServiceConfig) {
    cfg.service(get_new_comments_action);
}
