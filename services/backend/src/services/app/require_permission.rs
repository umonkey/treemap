use super::{Injected, UserId};
use crate::domain::iam::IamService;
use crate::types::{Error, Result};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;
use std::marker::PhantomData;
use std::ops::Deref;

/// A trait for defining permissions.
pub trait Permission {
    const NAME: &'static str;
}

/// An extractor that requires a specific permission.
pub struct RequirePermission<P: Permission>(pub u64, PhantomData<P>);

impl<P: Permission> Deref for RequirePermission<P> {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Permission + 'static> FromRequest for RequirePermission<P> {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let user_id_fut = UserId::from_request(&req, payload);
        let iam_service_fut = Injected::<IamService>::from_request(&req, payload);

        Box::pin(async move {
            let user_id = user_id_fut.await?;
            let iam_service = iam_service_fut.await?;

            iam_service.require_permission(*user_id, P::NAME).await?;

            Ok(RequirePermission(*user_id, PhantomData))
        })
    }
}

// Define permissions here or in respective domains.
pub struct PanoEdit;
impl Permission for PanoEdit {
    const NAME: &'static str = "pano:edit";
}

pub struct UserManage;
impl Permission for UserManage {
    const NAME: &'static str = "user:manage";
}
