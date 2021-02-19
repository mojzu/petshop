//! # petshop_server
//!
#[macro_use]
extern crate log;

use tonic::{transport::Server, Request, Response, Status};

use petshop_proto::petshop_server::{Petshop, PetshopServer};
use petshop_proto::{Category, Pet, Status as PetshopStatus, Tag};

#[derive(Debug, Default)]
pub struct MyPetshop {}

#[tonic::async_trait]
impl Petshop for MyPetshop {
    async fn pet_post(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        println!("pet_post request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }

    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        println!("pet_put request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = "0.0.0.0:5000".parse()?;
    let petshop = MyPetshop::default();

    debug!("listening on {}", addr);
    Server::builder()
        .add_service(PetshopServer::new(petshop))
        .serve(addr)
        .await?;

    Ok(())
}
