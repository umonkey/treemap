use crate::domain::comment::CommentService;
use crate::services::comment_loader::{CommentList, CommentLoader};
use crate::services::Injected;
use crate::types::Result;
use actix_web::get;
use actix_web::web::Json;

#[get("")]
pub async fn get_new_comments_action(
    comments_service: Injected<CommentService>,
    loader: Injected<CommentLoader>,
) -> Result<Json<CommentList>> {
    let comments = comments_service.get_new(100).await?;
    let res = loader.load(&comments).await?;
    Ok(Json(res))
}
