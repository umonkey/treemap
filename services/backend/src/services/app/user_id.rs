use super::AppState;
use crate::types::{Error, Result};
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Ready};
use std::ops::Deref;

/// An extractor for the authenticated user ID.
pub struct UserId(pub u64);

impl Deref for UserId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for UserId {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();

        let result = if let Some(state) = extensions.get::<AppState>() {
            state.get_user_id(req).map(UserId)
        } else if let Some(state) = req.app_data::<Data<AppState>>() {
            state.get_user_id(req).map(UserId)
        } else {
            Err(Error::Config("AppState not found in request".into()))
        };

        ready(result)
    }
}

/// An extractor for the optional authenticated user ID.
/// Returns 0 if the user is not authenticated.
/// Returns an error if the token is malformed.
pub struct OptionalUserId(pub u64);

impl Deref for OptionalUserId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for OptionalUserId {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();

        let state = if let Some(state) = extensions.get::<AppState>() {
            state
        } else if let Some(state) = req.app_data::<Data<AppState>>() {
            state
        } else {
            return ready(Err(Error::Config("AppState not found in request".into())));
        };

        let result = match state.get_user_id(req) {
            Ok(id) => Ok(OptionalUserId(id)),
            Err(Error::MissingAuthorizationHeader) => Ok(OptionalUserId(0)),
            Err(e) => Err(e),
        };

        ready(result)
    }
}
