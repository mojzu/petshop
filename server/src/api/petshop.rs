//! # Petshop
//!
use crate::internal::*;
use petshop_proto::api::petshop_server::Petshop;
use petshop_proto::api::{
    Category, Echo, FindByStatus, FindByTag, Pet, Pets, Status as PetStatus, Tag, User,
};
use petshop_proto::google::api::HttpBody;
use prost_types::Struct;
use std::fmt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

impl Api {
    /// Streaming async example task for echoing message on a timer
    #[tracing::instrument(skip(tx))]
    async fn streaming_ex_task(tx: mpsc::Sender<Result<Echo, Status>>, echo: Echo) {
        info!("streaming_ex_task echo {:?}", echo);
        for _ in 0..3 {
            info!("echo {:?}", echo);
            tx.send(Ok(echo.clone())).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        info!("echo stream done");
    }
}

#[tonic::async_trait]
impl Petshop for Api {
    #[tracing::instrument(skip(self))]
    async fn http_body_ex(&self, _request: Request<()>) -> Result<Response<HttpBody>, Status> {
        info!("http_body request");
        let body = HttpBody {
            content_type: "text/html".to_string(),
            data: "<h1>Hello, world!</h1>".into(),
            extensions: vec![],
        };
        Ok(Response::new(body))
    }

    #[tracing::instrument(skip(self))]
    async fn json_ex(&self, request: Request<Struct>) -> Result<Response<Struct>, Status> {
        info!("json request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument(skip(self))]
    async fn authentication_required_ex(
        &self,
        request: Request<()>,
    ) -> Result<Response<User>, Status> {
        info!("authentication_required request");
        let user = self.auth_required(&request)?;
        Ok(Response::new(user))
    }

    #[tracing::instrument(skip(self))]
    async fn validation_ex(&self, request: Request<User>) -> Result<Response<User>, Status> {
        info!("validation_ex request");

        let user = request.into_inner();
        self.validate(&user)?;

        Ok(Response::new(user))
    }

    #[tracing::instrument(skip(self))]
    async fn csrf_ex(&self, request: Request<()>) -> Result<Response<()>, Status> {
        info!("csrf_ex request");

        self.csrf.request_check(&request)?;

        self.csrf.response_used(Response::new(()))
    }

    type StreamingExStream = ReceiverStream<Result<Echo, Status>>;

    #[tracing::instrument(skip(self))]
    async fn streaming_ex(
        &self,
        request: Request<Echo>,
    ) -> Result<Response<Self::StreamingExStream>, Status> {
        info!("streaming request");

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(Self::streaming_ex_task(tx, request.into_inner()));

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn pet_post(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_post request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument(skip(self))]
    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_put request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument(skip(self))]
    async fn pet_find_by_status(
        &self,
        _request: Request<FindByStatus>,
    ) -> Result<Response<Pets>, Status> {
        info!("pet_find_by_status request");
        let pet = Pet {
            id: 1,
            category: Some(Category {
                id: 1,
                name: "CategoryName1".to_string(),
            }),
            name: "PetName1".to_string(),
            photo_urls: vec!["PhotoUrl1".to_string()],
            tags: vec![Tag {
                id: 1,
                name: "TagName1".to_string(),
            }],
            status: PetStatus::Pending as i32,
        };
        Ok(Response::new(Pets { pets: vec![pet] }))
    }

    #[tracing::instrument(skip(self))]
    async fn pet_find_by_tag(
        &self,
        _request: Request<FindByTag>,
    ) -> Result<Response<Pets>, Status> {
        info!("pet_find_by_tag request");
        let pet = Pet {
            id: 1,
            category: Some(Category {
                id: 1,
                name: "CategoryName2".to_string(),
            }),
            name: "PetName2".to_string(),
            photo_urls: vec!["PhotoUrl2".to_string()],
            tags: vec![Tag {
                id: 1,
                name: "TagName2".to_string(),
            }],
            status: PetStatus::Pending as i32,
        };
        Ok(Response::new(Pets { pets: vec![pet] }))
    }
}

impl fmt::Debug for Api {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Api").finish()
    }
}
