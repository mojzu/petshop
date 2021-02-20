use tonic::{Request, Response, Status};

use petshop_proto::petshop_server::Petshop;
use petshop_proto::{Category, FindByStatus, HttpBody, Pet, Pets, Status as PetStatus, Tag};

#[derive(Debug, Default)]
pub struct Api {}

#[tonic::async_trait]
impl Petshop for Api {
    async fn http_body(&self, request: Request<HttpBody>) -> Result<Response<HttpBody>, Status> {
        info!("http_body request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }

    async fn pet_post(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_post request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }

    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_put request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }

    async fn pet_find_by_status(
        &self,
        request: Request<FindByStatus>,
    ) -> Result<Response<Pets>, Status> {
        info!("pet_find_by_status request: {:?}", request);
        let pet = Pet {
            id: 1,
            category: Some(Category {
                id: 1,
                name: "CategoryName".to_string(),
            }),
            name: "PetName".to_string(),
            photo_urls: vec!["PhotoUrl".to_string()],
            tags: vec![Tag {
                id: 1,
                name: "TagName".to_string(),
            }],
            status: PetStatus::Pending as i32,
        };
        Ok(Response::new(Pets { pets: vec![pet] }))
    }
}
