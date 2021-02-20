use tonic::{Request, Response, Status};

use petshop_proto::Pet;
use petshop_proto::petshop_server::Petshop;

#[derive(Debug, Default)]
pub struct Api {}

#[tonic::async_trait]
impl Petshop for Api {
    async fn pet_post(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_post request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }

    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_put request: {:?}", request);
        Ok(Response::new(request.into_inner()))
    }
}
