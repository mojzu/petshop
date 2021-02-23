//! # petshop_server
//!
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use clap::{App, Arg};
use petshop_proto::petshop_server::PetshopServer;

use crate::api::Api;
use crate::config::Config;
use crate::internal::*;
use hyper::service::{make_service_fn, service_fn};

mod api;
mod config;
mod internal;
mod metrics;

/// Main
///
/// Simple command line interface for configuration file path argument (`-c` or `--config`).
/// Loads configuration from file (optional) and environment.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(NAME)
        .version(VERSION)
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let config_file = matches.value_of("config");
    let config = Config::load(config_file)?;
    config.init_panic_and_log();

    // Build gRPC health service
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<PetshopServer<Api>>().await;

    // Build API service
    let petshop = Api::from_config(&config);
    let petshop_internal = petshop.clone();

    // Build and serve tonic api server
    info!("api listening on {}", config.api_addr);
    let api_server = tonic::transport::Server::builder()
        .add_service(health_service)
        .add_service(PetshopServer::new(petshop))
        .serve_with_shutdown(config.api_addr, shutdown_signal());

    // Build and serve hyper internal server
    info!("internal listening on {}", config.internal_addr);
    let internal_service = make_service_fn(move |_| {
        let api = petshop_internal.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let api = api.clone();
                internal_http_request_response(api, req)
            }))
        }
    });
    let internal_server = hyper::Server::bind(&config.internal_addr)
        .serve(internal_service)
        .with_graceful_shutdown(shutdown_signal());

    // Await server termination via signal
    let (api_server, internal_server) = tokio::join!(api_server, internal_server);
    api_server?;
    internal_server?;
    Ok(())
}

/// Graceful shutdown signal handler
#[cfg(target_family = "unix")]
async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigint = signal(SignalKind::interrupt()).expect("SIGINT signal failure");
    let mut sigterm = signal(SignalKind::terminate()).expect("SIGTERM signal failure");
    let mut sigquit = signal(SignalKind::quit()).expect("SIGQUIT signal failure");

    let sig = tokio::select! {
        _ = sigint.recv() => { "SIGINT" }
        _ = sigterm.recv() => { "SIGTERM" }
        _ = sigquit.recv() => { "SIGQUIT" }
    };
    info!("received shutdown signal ({})", sig);
}

/// Graceful shutdown signal handler
#[cfg(not(target_family = "unix"))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("shutdown_signal failed");
    debug!("received shutdown signal");
}

// TODO: Prometheus, Kubernetes endpoints, other best practices?
// TODO: Docker compose test suite (for dev and CI?)

// TODO: Github actions to build docker images with versions (cd.yml?)
// TODO: Auth integrations, mtls and other options using envoy?
// TODO: Database integrations, migrations crate/binary?
// TODO: Rust docs output in dist? cargo make --no-workspace docs-flow
// TODO: Use alpine image for server docker image? Research podman?
// TODO: Running in k8s/nomad examples?
// TODO: Openapi doc generator from specification?
// TODO: Add examples for rust tests/examples/benches?
// TODO: More comments/explanation in proto and server files?
// TODO: Read the docs docker image for docs?
// <https://docs.readthedocs.io/en/stable/intro/getting-started-with-sphinx.html#quick-start-video>
// TODO: Double check how bytes is being deserialised from json in httpbody, add note for this (base64?)
// TODO: Handle SIGHUP to restart without exit?
