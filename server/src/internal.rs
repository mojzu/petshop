//! # Internal
//!
//! Some library types made public for easier use in modules.
//!
//! Internal HTTP server request handlers.
pub use crate::config::Config;
pub use crate::csrf::{Csrf, CsrfConfig, CsrfService};
pub use crate::jobs::Jobs;
pub use crate::metrics::{Metrics, MetricsService};
pub use crate::postgres::{PostgresClient, PostgresPool};
pub use crate::services::Api;
pub use anyhow::{Error, Result};
pub use chrono::Utc;
pub use std::convert::{TryFrom, TryInto};
pub use std::str::FromStr;
pub use std::sync::Arc;
pub use std::time::SystemTime;
pub use url::Url;

use hyper::{Body, Method, Request, Response, StatusCode};

/// Crate Name
pub static NAME: &str = env!("CARGO_PKG_NAME");

/// Crate Version
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate User Agent
pub static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub type HttpStatus = http::StatusCode;

pub type HttpHeaders = http::header::HeaderMap<http::header::HeaderValue>;

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum XError {
    #[error("configuration error `{0}`")]
    Config(String),

    #[error("jobs error `{0}`")]
    Jobs(String),

    #[error("internal uri error `{0}`")]
    InternalUri(String),

    #[error("serde json error")]
    SerdeJson(#[from] serde_json::Error),

    #[error("postgres config error")]
    PostgresConfig(#[from] deadpool_postgres::config::ConfigError),

    #[error("postgres pool error")]
    PostgresPool(#[from] deadpool_postgres::PoolError),

    #[error("postgres error")]
    Postgres(#[from] tokio_postgres::Error),
}

impl XError {
    pub fn config(message: &str) -> Self {
        Self::Config(message.to_string())
    }

    pub fn jobs(message: &str) -> Self {
        Self::Jobs(message.to_string())
    }

    pub fn internal_uri(uri: &str) -> Self {
        Self::InternalUri(uri.to_string())
    }
}

impl From<XError> for tonic::Status {
    fn from(err: XError) -> Self {
        // These errors come from modules like Postgres, where you
        // probably wouldn't want to include error details in the
        // response, log them here instead which will include
        // tracing information from the request handler
        //
        // <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
        // <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
        let err: Error = err.into();
        warn!("{:#}", err);

        tonic::Status::internal("error")
    }
}

/// Internal HTTP request handler for metrics and other private endpoints
#[tracing::instrument(skip(api))]
pub async fn http_request_handler(api: Api, req: Request<Body>) -> Result<Response<Body>> {
    let metrics = api.metrics();
    metrics.internal_counter_inc();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => liveness_request_response(),
        (&Method::GET, "/liveness") => liveness_request_response(),
        (&Method::GET, "/readiness") => readiness_request_response(&api).await,
        (&Method::GET, "/metrics") => metrics_request_response(&api),
        (_, uri) => Err(XError::internal_uri(uri).into()),
    }
    .or_else(|e| {
        // In case of an internal error do not send details
        // back to client, log them here instead
        //
        // <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
        // <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
        metrics.internal_error_counter_inc();
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
async fn readiness_request_response(api: &Api) -> Result<Response<Body>> {
    api.readiness().await?;
    liveness_request_response()
}

/// Prometheus metrics request handler
fn metrics_request_response(api: &Api) -> Result<Response<Body>> {
    let (content_type, buffer) = api.metrics().export();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .body(buffer.into())?)
}

/// Converts a serde derived Value into a prost Value
///
/// FIXME: It feels like there should be a cleaner way to do this, or to avoid
/// this conversion by converting a prost derived struct into its value
///
/// FIXME: Fix TFB warning - "An extra key(s) is being included with the db object: values"
/// It doesn't appear to be returned in the response, so is envoy adding this during transcoding?
pub fn serde_into_prost_value(value: serde_json::Value) -> prost_types::Value {
    let kind: prost_types::value::Kind = match value {
        serde_json::Value::Null => prost_types::value::Kind::NullValue(0),
        serde_json::Value::Bool(x) => prost_types::value::Kind::BoolValue(x),
        serde_json::Value::Number(x) => prost_types::value::Kind::NumberValue(x.as_f64().unwrap()),
        serde_json::Value::String(x) => prost_types::value::Kind::StringValue(x),
        serde_json::Value::Array(x) => {
            let mut v = Vec::new();
            for value in x {
                v.push(serde_into_prost_value(value));
            }
            prost_types::value::Kind::ListValue(prost_types::ListValue { values: v })
        }
        serde_json::Value::Object(x) => {
            let mut fields = std::collections::BTreeMap::new();
            for (key, value) in x {
                fields.insert(key, serde_into_prost_value(value));
            }
            prost_types::value::Kind::StructValue(prost_types::Struct { fields })
        }
    };
    prost_types::Value { kind: Some(kind) }
}

/// Returns grpc-status code from response headers, if header is not present assumed to be 0
pub fn http_headers_grpc_status(headers: &HttpHeaders) -> tonic::Code {
    match headers.get("grpc-status") {
        Some(header) => match header.to_str() {
            Ok(value) => match value.parse::<i32>() {
                Ok(value) => tonic::Code::from_i32(value),
                Err(_) => tonic::Code::Unknown,
            },
            Err(_) => tonic::Code::Unknown,
        },
        None => tonic::Code::Ok,
    }
}
