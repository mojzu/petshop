//! # Metrics
//!
use crate::internal::*;
use opentelemetry::metrics::{BoundCounter, BoundValueRecorder};
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};
use std::fmt;

pub use service::MetricsService;

mod service;

/// Metrics
///
/// Can add more metrics here for collection
pub struct Metrics {
    exporter: PrometheusExporter,
    ready: BoundValueRecorder<'static, u64>,
    counter: BoundCounter<'static, u64>,
    error_counter: BoundCounter<'static, u64>,
    latency: BoundValueRecorder<'static, f64>,
    csrf_error_counter: BoundCounter<'static, u64>,
    validate_error_counter: BoundCounter<'static, u64>,
    internal_counter: BoundCounter<'static, u64>,
    internal_error_counter: BoundCounter<'static, u64>,
    postgres_ready: BoundValueRecorder<'static, u64>,
}

impl Metrics {
    pub fn from_config(config: &Config) -> Self {
        let exporter = opentelemetry_prometheus::exporter()
            .with_registry(prometheus::default_registry().clone())
            .init();
        let meter = opentelemetry::global::meter(NAME);
        let name = &config.metrics_name;

        let ready = meter
            .u64_value_recorder(format!("{}.api_ready", name))
            .with_description("1 if API is ready, else 0.")
            .init()
            .bind(&[]);
        let counter = meter
            .u64_counter(format!("{}.api_counter_total", name))
            .with_description("Total number of API server requests made.")
            .init()
            .bind(&[]);
        let error_counter = meter
            .u64_counter(format!("{}.api_error_counter_total", name))
            .with_description("Total number of API server errors.")
            .init()
            .bind(&[]);
        let latency = meter
            .f64_value_recorder(format!("{}.api_latency_seconds", name))
            .with_description("The API server request latencies in seconds.")
            .init()
            .bind(&[]);
        let csrf_error_counter = meter
            .u64_counter(format!("{}.api_csrf_error_counter_total", name))
            .with_description("Total number of API server CSRF check errors.")
            .init()
            .bind(&[]);
        let validate_error_counter = meter
            .u64_counter(format!("{}.api_validate_error_counter_total", name))
            .with_description("Total number of API server validation check errors.")
            .init()
            .bind(&[]);

        let internal_counter = meter
            .u64_counter(format!("{}.internal_counter_total", name))
            .with_description("Total number of internal HTTP server requests made.")
            .init()
            .bind(&[]);
        let internal_error_counter = meter
            .u64_counter(format!("{}.internal_error_counter_total", name))
            .with_description("Total number of internal HTTP server errors.")
            .init()
            .bind(&[]);

        let postgres_ready = meter
            .u64_value_recorder(format!("{}.postgres_ready", name))
            .with_description("1 if postgres is ready, else 0.")
            .init()
            .bind(&[]);

        Self {
            exporter,
            ready,
            counter,
            error_counter,
            latency,
            csrf_error_counter,
            validate_error_counter,
            internal_counter,
            internal_error_counter,
            postgres_ready,
        }
    }

    #[inline]
    pub fn api_ready(&self, ready: bool) {
        let value = if ready { 1 } else { 0 };
        self.ready.record(value);
    }

    #[inline]
    pub fn csrf_error_counter_inc(&self) {
        self.csrf_error_counter.add(1);
    }

    #[inline]
    pub fn validate_error_counter_inc(&self) {
        self.validate_error_counter.add(1);
    }

    #[inline]
    pub fn internal_counter_inc(&self) {
        self.internal_counter.add(1);
    }

    #[inline]
    pub fn internal_error_counter_inc(&self) {
        self.internal_error_counter.add(1);
    }

    #[inline]
    pub fn postgres_ready(&self, ready: bool) {
        let value = if ready { 1 } else { 0 };
        self.postgres_ready.record(value);
    }

    #[inline]
    pub fn service_request_handler(&self) -> SystemTime {
        self.counter.add(1);
        SystemTime::now()
    }

    #[inline]
    pub fn service_response_handler(&self, start: SystemTime, headers: Option<&HttpHeaders>) {
        if let Some(headers) = headers {
            if http_headers_grpc_status(headers) != tonic::Code::Ok {
                self.error_counter.add(1);
            }
        }

        self.latency
            .record(start.elapsed().map_or(0.0, |d| d.as_secs_f64()));
    }

    /// Export metrics in prometheus exposition format
    ///
    /// Include application metrics and process metrics
    pub fn export(&self) -> (String, Vec<u8>) {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.exporter.registry().gather();

        encoder
            .encode(&metric_families, &mut buffer)
            .expect("encode metrics failed");

        (encoder.format_type().to_string(), buffer)
    }
}

impl fmt::Debug for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Metrics").finish()
    }
}
