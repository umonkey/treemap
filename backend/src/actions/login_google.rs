use actix_web::{post, web::Data, web::Json};

use crate::Result;
use crate::objects::{LoginGoogleRequest, LoginResponse};
use crate::services::AppState;

#[post("/v1/login/google")]
pub async fn login_google(
    state: Data<AppState>,
    payload: Json<LoginGoogleRequest>,
) -> Result<Json<LoginResponse>> {
    let req = LoginGoogleRequest {
        token: payload.token.clone(),
    };

    let login = state.login_google(req).await?;
    Ok(Json(login))
}