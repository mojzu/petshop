use crate::internal::*;
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService};
use tower::Service;

/// Service interceptor to collect counter and latency metrics
#[derive(Debug, Clone)]
pub struct MetricsService<S> {
    metrics: Arc<Metrics>,
    inner: S,
}

impl<S> MetricsService<S> {
    pub fn wrap(metrics: Arc<Metrics>, api: S) -> Self {
        Self {
            metrics,
            inner: api,
        }
    }
}

impl<S> Service<HyperRequest<Body>> for MetricsService<S>
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

    fn call(&mut self, req: HyperRequest<Body>) -> Self::Future {
        let mut svc = self.inner.clone();
        let metrics = self.metrics.clone();

        Box::pin(async move {
            metrics.api_counter_inc();
            let request_start = SystemTime::now();

            let res = svc.call(req).await;

            metrics.api_latency_record(request_start);
            res
        })
    }
}

impl<S: NamedService> NamedService for MetricsService<S> {
    const NAME: &'static str = S::NAME;
}
