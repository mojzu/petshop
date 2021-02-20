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

    let addr = "0.0.0.0:5000".parse()?;
    let petshop = Api::default();

    debug!("listening on {}", addr);
    Server::builder()
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
