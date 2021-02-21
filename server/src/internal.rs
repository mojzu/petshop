//! # Internal
//!
//! Some library types made public for easier use in modules.
//!
//! Internal HTTP server request handlers.
pub use std::convert::{TryFrom, TryInto};

pub use anyhow::{Error, Result};
pub use chrono::Utc;
use hyper::{Body, Method, Request, Response, StatusCode};

/// Crate Name
pub static NAME: &str = env!("CARGO_PKG_NAME");

/// Crate Version
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Internal HTTP request handler for metrics and other private endpoints
pub async fn internal_http_request_response(req: Request<Body>) -> Result<Response<Body>> {
    // TODO: Handle other requests here?
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => metrics_request_response().await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())
            .expect("response builder failure")),
    }
}

async fn metrics_request_response() -> Result<Response<Body>> {
    // TODO: Return prometheus metrics here
    let metrics: String = "".to_string();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(metrics.into())
        .expect("response builder failure"))
}
