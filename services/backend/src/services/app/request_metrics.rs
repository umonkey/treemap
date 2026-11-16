//! This middleware reports how long each request takes as a Sentry metric.
//!
//! Every request emits `http.server.request.duration`, and requests at or above
//! `SLOW_REQUEST` also increment `http.server.request.slow`. Both are tagged
//! with the route pattern, the method and the status code, so a slow endpoint is
//! identifiable without reading traces.
//!
//! The measured duration covers the request up to the response head, not the
//! body streaming that follows it.
//!
//! The middleware is registered on the whole app, so unmatched requests, static
//! files and every scope are covered.

use crate::utils::metrics::{normalize_route, record_request, UNMATCHED_ROUTE};
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::time::Instant;

pub struct RequestMetrics;

impl<S, B> Transform<S, ServiceRequest> for RequestMetrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestMetricsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestMetricsMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestMetricsMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestMetricsMiddleware<S>
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
        let started = Instant::now();
        let method = req.method().as_str().to_string();

        Box::pin(async move {
            let result = service.call(req).await;

            let (pattern, status) = match &result {
                Ok(res) => (res.request().match_pattern(), res.status().as_u16()),
                Err(error) => (None, error.as_response_error().status_code().as_u16()),
            };

            let route = pattern
                .as_deref()
                .map_or_else(|| UNMATCHED_ROUTE.to_string(), normalize_route);

            record_request(&method, &route, status, started.elapsed());

            result
        })
    }
}
