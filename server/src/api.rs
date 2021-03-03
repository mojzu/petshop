//! # API
//!
use crate::internal::*;
use petshop_proto::api::v1::petshop_server::Petshop;
use petshop_proto::api::v1::{
    Category, Echo, FindByStatus, FindByTag, Pet, Pets, Queries, Status as PetStatus, Tag, User,
    World,
};
use petshop_proto::google::api::HttpBody;
use prost_types::{ListValue, Struct, Value};
use std::fmt;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

/// API Server
#[derive(Clone)]
pub struct Api {
    metrics: Arc<Metrics>,
    postgres: Arc<Postgres>,
    shutdown: Arc<broadcast::Sender<bool>>,
}

impl Api {
    pub fn from_config(
        config: &Config,
        shutdown_tx: broadcast::Sender<bool>,
    ) -> Result<Self, XError> {
        let metrics = Arc::new(Metrics::from_config(config));
        Ok(Self {
            metrics: metrics.clone(),
            postgres: Arc::new(Postgres::from_config(config, metrics)?),
            shutdown: Arc::new(shutdown_tx),
        })
    }

    pub fn metrics(&self) -> Arc<Metrics> {
        self.metrics.clone()
    }

    pub fn postgres(&self) -> Arc<Postgres> {
        self.postgres.clone()
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
    pub async fn readiness(&self) -> Result<(), XError> {
        let postgres_ready = self.postgres.readiness().await;
        self.metrics.api_ready_set(postgres_ready.is_ok());
        postgres_ready?;
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

    /// Validates request using derived validate method
    fn validate<T: validator::Validate>(&self, request: &T) -> Result<(), Status> {
        match request.validate() {
            Ok(_) => Ok(()),
            Err(e) => {
                let serialised = serde_json::to_string(&e).expect("serialisation failed");
                let encoded = base64::encode(serialised);
                Err(Status::invalid_argument(encoded))
            }
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

    #[tracing::instrument(skip(self))]
    async fn tfb_queries(&self, request: Queries) -> Result<Response<ListValue>, Status> {
        let queries = if request.queries < 1 {
            1
        } else if request.queries > 500 {
            500
        } else {
            request.queries
        };
        let worlds = self.postgres.db_world_queries(queries).await?;

        let values: Vec<Value> = worlds
            .into_iter()
            .map(|x| {
                let serde_value = json!({ "id": x.id, "randomNumber": x.random_number });
                serde_into_prost_value(serde_value)
            })
            .collect();
        let v = ListValue { values };

        Ok(Response::new(v))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_updates(&self, request: Queries) -> Result<Response<ListValue>, Status> {
        let queries = if request.queries < 1 {
            1
        } else if request.queries > 500 {
            500
        } else {
            request.queries
        };
        let worlds = self.postgres.db_world_updates(queries).await?;

        let values: Vec<Value> = worlds
            .into_iter()
            .map(|x| {
                let serde_value = json!({ "id": x.id, "randomNumber": x.random_number });
                serde_into_prost_value(serde_value)
            })
            .collect();
        let v = ListValue { values };

        Ok(Response::new(v))
    }
}

#[tonic::async_trait]
impl Petshop for Api {
    #[tracing::instrument(skip(self))]
    async fn http_body_ex(&self, request: Request<()>) -> Result<Response<HttpBody>, Status> {
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
        let user = self.user_required(&request)?;
        Ok(Response::new(user))
    }

    #[tracing::instrument(skip(self))]
    async fn validation_ex(&self, request: Request<User>) -> Result<Response<User>, Status> {
        info!("validation_ex request");
        let user = request.into_inner();
        self.validate(&user)?;
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
    async fn tfb_json(&self, request: Request<()>) -> Result<Response<Echo>, Status> {
        Ok(Response::new(Echo {
            message: "Hello, world!".to_string(),
        }))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_plaintext(&self, request: Request<()>) -> Result<Response<HttpBody>, Status> {
        let body = HttpBody {
            content_type: "text/plain".to_string(),
            data: "Hello, world!".into(),
            extensions: vec![],
        };
        Ok(Response::new(body))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_db(&self, request: Request<()>) -> Result<Response<World>, Status> {
        let world = self.postgres.db_world().await?;
        Ok(Response::new(world))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_queries1(&self, request: Request<Queries>) -> Result<Response<ListValue>, Status> {
        Ok(self.tfb_queries(request.into_inner()).await?)
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_queries2(&self, request: Request<Queries>) -> Result<Response<ListValue>, Status> {
        Ok(self.tfb_queries(request.into_inner()).await?)
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_updates1(&self, request: Request<Queries>) -> Result<Response<ListValue>, Status> {
        Ok(self.tfb_updates(request.into_inner()).await?)
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_updates2(&self, request: Request<Queries>) -> Result<Response<ListValue>, Status> {
        Ok(self.tfb_updates(request.into_inner()).await?)
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
