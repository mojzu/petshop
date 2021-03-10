//! # CSRF Service
//!
use crate::internal::*;
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService};
use tower::Service;

/// Service interceptor to manage csrf tokens
#[derive(Debug, Clone)]
pub struct CsrfService<S> {
    csrf: Arc<Csrf>,
    inner: S,
}

impl<S> CsrfService<S> {
    pub fn wrap(csrf: Arc<Csrf>, api: S) -> Self {
        Self { csrf, inner: api }
    }
}

impl<S> Service<HyperRequest<Body>> for CsrfService<S>
where
    S: Service<HyperRequest<Body>, Response = HyperResponse<BoxBody>>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: HyperRequest<Body>) -> Self::Future {
        let mut svc = self.inner.clone();
        let csrf = self.csrf.clone();

        Box::pin(async move {
            let xsrf_token = csrf.service_request_handler(req.headers_mut());

            let mut res = svc.call(req).await?;

            csrf.service_response_handler(xsrf_token, res.status(), res.headers_mut());

            Ok(res)
        })
    }
}

impl<S: NamedService> NamedService for CsrfService<S> {
    const NAME: &'static str = S::NAME;
}
