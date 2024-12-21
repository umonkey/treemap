use actix_web::{post, web::Data, web::Json};

use crate::services::AppState;
use crate::types::{LoginGoogleRequest, LoginResponse, Result};

#[post("/v1/login/google")]
pub async fn login_google_action(
    state: Data<AppState>,
    payload: Json<LoginGoogleRequest>,
) -> Result<Json<LoginResponse>> {
    let login = state
        .login_google_handler
        .handle(LoginGoogleRequest {
            token: payload.token.clone(),
        })
        .await?;

    Ok(Json(login))
}
