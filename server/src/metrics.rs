use opentelemetry::metrics::{BoundCounter, BoundValueRecorder};
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};

use crate::internal::*;

/// Metrics
///
/// Can add more metrics here for collection
pub struct Metrics {
    exporter: PrometheusExporter,
    api_counter: BoundCounter<'static, u64>,
    api_latency: BoundValueRecorder<'static, f64>,
}

impl Metrics {
    pub fn from_config(_config: &Config) -> Self {
        let exporter = opentelemetry_prometheus::exporter()
            .with_registry(prometheus::default_registry().clone())
            .init();
        let meter = opentelemetry::global::meter(NAME);

        let api_counter = meter
            .u64_counter("api_request_counter_total")
            .with_description("Total number of API requests made.")
            .init()
            .bind(&[]);
        let api_latency = meter
            .f64_value_recorder("api_request_latency_seconds")
            .with_description("The API request latencies in seconds.")
            .init()
            .bind(&[]);

        Self {
            exporter,
            api_counter,
            api_latency,
        }
    }

    pub fn api_counter_inc(&self) {
        self.api_counter.add(1);
    }

    pub fn api_latency_record(&self, time: SystemTime) {
        self.api_latency
            .record(time.elapsed().map_or(0.0, |d| d.as_secs_f64()));
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
            .expect("encode failed");

        (encoder.format_type().to_string(), buffer)
    }
}
