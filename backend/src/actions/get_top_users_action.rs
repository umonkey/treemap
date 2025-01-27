use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json};

#[get("/v1/stats/top-users")]
pub async fn get_top_users_action(state: Data<AppState>) -> Result<Json<UserList>> {
    let res = state.get_top_users_handler.handle().await?;
    Ok(Json(res))
}
