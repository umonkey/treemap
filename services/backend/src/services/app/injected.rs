//! This module provides the `Injected<T>` extractor for Actix-Web.
//!
//! It allows for automatic dependency injection of any type that implements
//! the `Injectable` trait directly into web action arguments.
//!
//! # Example
//!
//! ```rust
//! #[get("")]
//! pub async fn my_action(service: Injected<MyService>) -> Result<Json<Data>> {
//!     let result = service.do_something().await?;
//!     Ok(Json(result))
//! }
//! ```

use super::{AppState, ContextExt, Injectable};
use crate::types::{Error, Result};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};
use std::ops::Deref;

/// A wrapper for types that can be automatically injected from the `AppState`.
pub struct Injected<T: Injectable>(pub T);

impl<T: Injectable> Deref for Injected<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Injectable + 'static> FromRequest for Injected<T> {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        use actix_web::web::Data;
        use actix_web::HttpMessage;

        let extensions = req.extensions();

        let result = if let Some(state) = extensions.get::<AppState>() {
            state.build::<T>().map(Injected)
        } else if let Some(state) = req.app_data::<Data<AppState>>() {
            state.build::<T>().map(Injected)
        } else {
            Err(Error::Config("AppState not found in request".into()))
        };

        ready(result)
    }
}
