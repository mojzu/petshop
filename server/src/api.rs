//! # API
//!
use std::fmt;

use prost_types::Struct;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use petshop_proto::{
    Category, Echo, FindByStatus, FindByTag, HttpBody, Pet, Pets, Status as PetStatus, Tag, User,
};
use petshop_proto::petshop_server::Petshop;

use crate::internal::*;

/// API Errors
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    // #[error("example error")]
// Example,
}

/// API Server
#[derive(Clone)]
pub struct Api {
    metrics: Arc<Metrics>,
    shutdown: Arc<broadcast::Sender<bool>>,
}

impl Api {
    pub fn from_config(config: &Config, shutdown_tx: broadcast::Sender<bool>) -> Self {
        Self {
            metrics: Arc::new(Metrics::from_config(config)),
            shutdown: Arc::new(shutdown_tx),
        }
    }

    pub fn metrics(&self) -> Arc<Metrics> {
        self.metrics.clone()
    }

    /// Sends shutdown signal to stop application
    ///
    /// This lets the application trigger a graceful exit rather than panicking
    pub fn shutdown(&self) {
        self.shutdown.send(true).expect("shutdown failed");
    }

    /// Returns an error if requests can not be served
    ///
    /// [More information on liveness/readiness probes](https://blog.colinbreck.com/kubernetes-liveness-and-readiness-probes-how-to-avoid-shooting-yourself-in-the-foot/)
    pub async fn readiness(&self) -> Result<()> {
        // Err(ApiError::Example.into())
        Ok(())
    }

    /// Parses request metadata to extract authenticated user (works with authz example)
    fn user_required(&self, request: &Request<()>) -> Result<User, Status> {
        let email = request.metadata().get("x-auth-request-email");
        let user = request.metadata().get("x-auth-request-user");
        match (email, user) {
            (Some(email), Some(user)) => match (email.to_str(), user.to_str()) {
                (Ok(email), Ok(user)) => Ok(User {
                    email: email.to_string(),
                    name: user.to_string(),
                }),
                _ => Err(Status::unauthenticated("user authentication failed")),
            },
            _ => Err(Status::unauthenticated("user authentication failed")),
        }
    }

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
    async fn http_body_ex(&self, request: Request<HttpBody>) -> Result<Response<HttpBody>, Status> {
        info!("http_body request");
        Ok(Response::new(request.into_inner()))
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
        let user = self.user_required(&request)?;
        Ok(Response::new(user))
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
        request: Request<FindByStatus>,
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
    async fn pet_find_by_tag(&self, request: Request<FindByTag>) -> Result<Response<Pets>, Status> {
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
