//! # API
//!
use std::fmt;

use tokio::sync::broadcast;
use tonic::{Request, Response, Status};

use petshop_proto::{
    Category, FindByStatus, FindByTag, HttpBody, Pet, Pets, Status as PetStatus, Tag,
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
}

#[tonic::async_trait]
impl Petshop for Api {
    #[tracing::instrument]
    async fn http_body(&self, request: Request<HttpBody>) -> Result<Response<HttpBody>, Status> {
        info!("http_body request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument]
    async fn pet_post(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_post request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument]
    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        info!("pet_put request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument]
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

    #[tracing::instrument]
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
