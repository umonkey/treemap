use crate::services::AppState;
use crate::types::{GoogleAuthCallbackPayload, Result};
use actix_web::{post, web::Data, web::Form, web::Redirect};

#[post("/v3/login/google")]
pub async fn login_google_v3(
    state: Data<AppState>,
    payload: Form<GoogleAuthCallbackPayload>,
) -> Result<Redirect> {
    let redirect = state
        .login_google_v3(GoogleAuthCallbackPayload {
            access_token: payload.access_token.clone(),
            state: payload.state.clone(),
        })
        .await?;

    Ok(Redirect::to(redirect).see_other())
}
