#[macro_use]
extern crate log;

use tonic::{transport::Server, Request, Response, Status};

use petshop_proto::petshop_server::{Petshop, PetshopServer};
use petshop_proto::{Category, Pet, Status as PetshopStatus, Tag};

#[derive(Debug, Default)]
pub struct MyPetshop {}

#[tonic::async_trait]
impl Petshop for MyPetshop {
    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        println!("Got a request: {:?}", request);

        let reply = Pet {
            id: 42,
            category: Some(Category {
                id: 12,
                name: "category1".to_string(),
            }),
            name: "pet1".to_string(),
            photo_urls: vec!["photoUrl1".to_string()],
            status: PetshopStatus::Available as i32,
            tags: vec![Tag {
                id: 52,
                name: "tag1".to_string(),
            }],
        };

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
