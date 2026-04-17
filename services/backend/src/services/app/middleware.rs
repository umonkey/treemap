//! This middleware handles automatic database transactions for each request.
//!
//! It starts a transaction at the beginning of the request and commits it
//! if the response is successful (2xx or 3xx). This significantly speeds up
//! actions that perform 2 or more database queries by reducing the number
//! of disk synchronizations.

use crate::services::AppState;
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;

pub struct Transaction;

impl<S, B> Transform<S, ServiceRequest> for Transaction
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TransactionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TransactionMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct TransactionMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TransactionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let state = req.app_data::<Data<AppState>>().cloned();

        Box::pin(async move {
            let state = state.ok_or_else(|| {
                actix_web::error::ErrorInternalServerError("AppState missing in app_data")
            })?;

            // Optimization: Skip transactions for OPTIONS and GET requests.
            // This reduces connection churn and avoids unnecessary locks.
            let method = req.method().clone();
            let skip_transaction = method == actix_web::http::Method::OPTIONS
                || method == actix_web::http::Method::GET;

            if skip_transaction {
                return service.call(req).await;
            }

            // Start a new session (transaction).
            let session_state = state
                .session()
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            // Store the transactional state in request extensions.
            req.extensions_mut().insert(session_state.clone());

            let res = service.call(req).await?;

            // If the response is successful, commit the transaction.
            if res.status().is_success() || res.status().is_redirection() {
                session_state
                    .database
                    .commit()
                    .await
                    .map_err(actix_web::error::ErrorInternalServerError)?;
            }

            Ok(res)
        })
    }
}
