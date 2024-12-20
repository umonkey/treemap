use crate::services::AppState;
use crate::types::{CommentList, Result};
use actix_web::{get, web::Data, web::Json};

#[get("/v1/comments")]
pub async fn get_comments(state: Data<AppState>) -> Result<Json<CommentList>> {
    let comments = state.get_recent_comments(100).await?;
    Ok(Json(comments))
}
