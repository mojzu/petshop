//! # Internal
//!
//! Some library types made public for easier use in modules.
//!
//! Internal HTTP server request handlers.
pub use crate::api::Api;
pub use crate::config::Config;
pub use crate::metrics::{Metrics, MetricsService};
pub use crate::postgres::Postgres;
pub use anyhow::{Error, Result};
pub use chrono::Utc;
pub use std::convert::{TryFrom, TryInto};
pub use std::sync::Arc;
pub use std::time::SystemTime;

use hyper::{Body, Method, Request, Response, StatusCode};

/// Crate Name
pub static NAME: &str = env!("CARGO_PKG_NAME");

/// Crate Version
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate User Agent
pub static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum XError {
    #[error("configuration error `{0}`")]
    Config(String),

    #[error("postgres config error")]
    PostgresConfig(#[from] deadpool_postgres::config::ConfigError),

    #[error("postgres pool error")]
    PostgresPool(#[from] deadpool_postgres::PoolError),

    #[error("postgres error")]
    Postgres(#[from] tokio_postgres::Error),
}

impl XError {
    pub fn config(message: &str) -> Self {
        Self::Config(message.to_owned())
    }
}

impl From<XError> for tonic::Status {
    fn from(err: XError) -> Self {
        tonic::Status::internal(err.to_string())
    }
}

/// Internal HTTP request handler for metrics and other private endpoints
#[tracing::instrument(skip(api))]
pub async fn http_request_handler(api: Api, req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => liveness_request_response(),
        (&Method::GET, "/liveness") => liveness_request_response(),
        (&Method::GET, "/readiness") => readiness_request_response(api).await,
        (&Method::GET, "/metrics") => metrics_request_response(api),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("not found".into())?),
    }
    .or_else(|e| {
        warn!("{:#}", e);

        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error".into())?)
    })
}

/// Kubernetes liveness request handler
fn liveness_request_response() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("ok".into())?)
}

/// Kubernetes readiness request handler
async fn readiness_request_response(api: Api) -> Result<Response<Body>> {
    info!("checking readiness");
    api.readiness().await?;
    liveness_request_response()
}

/// Prometheus metrics request handler
fn metrics_request_response(api: Api) -> Result<Response<Body>> {
    info!("exporting metrics");
    let (content_type, buffer) = api.metrics().export();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .body(buffer.into())?)
}
