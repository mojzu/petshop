//! # API
//!
use crate::internal::*;
use petshop_proto::api::petshop_server::Petshop;
use petshop_proto::api::{
    Category, Echo, FindByStatus, FindByTag, Pet, Pets, Status as PetStatus, Tag, User,
};
use petshop_proto::google::api::HttpBody;
use prost_types::Struct;
use std::fmt;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

/// API Server
#[derive(Clone)]
pub struct Api {
    pub metrics: Arc<Metrics>,
    pub csrf: Arc<Csrf>,
    pub postgres: Arc<PostgresPool>,
    pub shutdown: Arc<broadcast::Sender<bool>>,

    /// This is only here for TFB fortunes endpoint
    pub tfb_handlebars: Arc<handlebars::Handlebars<'static>>,
}

const TFB_FORTUNES_HTML: &str = "<!DOCTYPE html>
<html>
<head><title>Fortunes</title></head>
<body>
<table>
<tr><th>id</th><th>message</th></tr>
{{#each rows}}
<tr><td>{{id}}</td><td>{{message}}</td></tr>
{{/each}}
</table>
</body>
</html>";

impl Api {
    pub fn from_config(
        config: &Config,
        shutdown_tx: broadcast::Sender<bool>,
    ) -> Result<Self, XError> {
        let metrics = Arc::new(Metrics::from_config(config));

        let mut tfb_handlebars = handlebars::Handlebars::new();
        tfb_handlebars
            .register_template_string("tfb_fortunes", TFB_FORTUNES_HTML)
            .expect("register template failed");

        Ok(Self {
            metrics: metrics.clone(),
            csrf: Arc::new(Csrf::from_config(config, metrics.clone())),
            postgres: Arc::new(PostgresPool::from_config(config, metrics)?),
            shutdown: Arc::new(shutdown_tx),
            tfb_handlebars: Arc::new(tfb_handlebars),
        })
    }

    pub fn metrics(&self) -> Arc<Metrics> {
        self.metrics.clone()
    }

    pub fn csrf(&self) -> Arc<Csrf> {
        self.csrf.clone()
    }

    pub fn postgres(&self) -> Arc<PostgresPool> {
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
        self.metrics.api_ready(postgres_ready.is_ok());
        postgres_ready?;
        Ok(())
    }

    /// Parses request metadata to extract authenticated user (works with auth example)
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

    /// Parses request metadata to return authenticated user, which may be provided by oauth2-proxy
    /// headers or by the authorization header
    ///
    /// FIXME: This is a placeholder function that accepts any authorization header as valid to demonstrate
    /// supporting authentication via oauth2-proxy (for users) or via a header (for computers)
    ///
    /// In the auth example, this is made functional by adding an envoy listener that does not use the
    /// ext_authz filter, so requests are still passed upstream where they can be checked by this function
    ///
    /// This example assumes that the application is going to manage/verify its own API keys and that
    /// all private endpoints will call this function
    ///
    /// <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#api-keys>
    fn auth_required(&self, request: &Request<()>) -> Result<User, Status> {
        let auth = request.metadata().get("authorization");
        match auth {
            Some(auth) => match auth.to_str() {
                Ok(auth) => Ok(User {
                    email: "apiconsumer@petshop.com".to_string(),
                    name: auth.to_string(),
                }),
                _ => self.user_required(request),
            },
            _ => self.user_required(request),
        }
    }

    /// Validates request using derived validate method, logs validation errors
    /// and returns serialised validation errors in message (base64 encoded)
    ///
    /// <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
    /// <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
    fn validate<T: validator::Validate>(&self, request: &T) -> Result<(), Status> {
        match request.validate() {
            Ok(_) => Ok(()),
            Err(err) => {
                let serialised = serde_json::to_string(&err).map_err(XError::SerdeJson)?;
                let encoded = base64::encode(serialised);

                self.metrics.validate_error_counter_inc();
                let err: Error = err.into();
                warn!("{:#}", err);

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