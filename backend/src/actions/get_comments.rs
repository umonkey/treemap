use crate::services::AppState;
use crate::types::{CommentList, Result};
use actix_web::{get, web::Data, web::Json};

#[get("/v1/comments")]
pub async fn get_new_comments(state: Data<AppState>) -> Result<Json<CommentList>> {
    let res = state.get_new_comments_handler.handle(100).await?;
    Ok(Json(res))
}
