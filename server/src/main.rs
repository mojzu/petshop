//! # petshop_server
//!
#[macro_use]
extern crate log;

use tonic::transport::Server;

use api::Api;
use petshop_proto::petshop_server::PetshopServer;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<PetshopServer<Api>>().await;

    let addr = "0.0.0.0:5000".parse()?;
    let petshop = Api::default();

    debug!("listening on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(PetshopServer::new(petshop))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}

#[cfg(target_family = "unix")]
async fn shutdown_signal() {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("shutdown_signal failed")
        .recv()
        .await;
    debug!("received shutdown signal");
}

#[cfg(not(target_family = "unix"))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("shutdown_signal failed");
    debug!("received shutdown signal");
}

// TODO: Github actions to build docker images with versions (cd.yml?)
// TODO: Docker compose test suite (for dev and CI?)
// TODO: Way of testing typescript clients, parcel for open in browser/run tests using TS?
// TODO: Change envoy config to support grpc-web and transcoding? Test this works
// TODO: Prometheus, Kubernetes endpoints, other best practices?
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
// TODO: Better method of caching/vendoring rust docker images to reduce compile times
