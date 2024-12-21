use crate::services::AppState;
use crate::types::{LoginGoogleRequest, LoginResponse, Result};
use actix_web::{post, web::Data, web::Json};

#[post("/v2/login/google")]
pub async fn login_google_v2_action(
    state: Data<AppState>,
    payload: Json<LoginGoogleRequest>,
) -> Result<Json<LoginResponse>> {
    let login = state
        .login_google_v2_handler
        .handle(LoginGoogleRequest {
            token: payload.token.clone(),
        })
        .await?;

    Ok(Json(login))
}
