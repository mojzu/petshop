//! # Example
//!
use crate::internal::*;
use petshop_proto::api::example_server::Example;
use petshop_proto::api::{Echo, User};
use petshop_proto::google::api::HttpBody;
use prost_types::Struct;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

impl Api {
    /// Streaming async example task for echoing message on a timer
    #[tracing::instrument(skip(tx))]
    async fn streaming_task(tx: mpsc::Sender<Result<Echo, Status>>, echo: Echo) {
        info!("streaming_task echo {:?}", echo);
        for _ in 0..3 {
            info!("echo {:?}", echo);
            tx.send(Ok(echo.clone())).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        info!("echo stream done");
    }
}

#[tonic::async_trait]
impl Example for Api {
    #[tracing::instrument(skip(self))]
    async fn http_body(&self, _request: Request<()>) -> Result<Response<HttpBody>, Status> {
        info!("http_body request");
        let body = HttpBody {
            content_type: "text/html".to_string(),
            data: "<h1>Hello, world!</h1>".into(),
            extensions: vec![],
        };
        Ok(Response::new(body))
    }

    #[tracing::instrument(skip(self))]
    async fn json(&self, request: Request<Struct>) -> Result<Response<Struct>, Status> {
        info!("json request");
        Ok(Response::new(request.into_inner()))
    }

    #[tracing::instrument(skip(self))]
    async fn authentication_required(
        &self,
        request: Request<()>,
    ) -> Result<Response<User>, Status> {
        info!("authentication_required request");
        let user = self.auth_required(&request)?;
        Ok(Response::new(user))
    }

    #[tracing::instrument(skip(self))]
    async fn validation(&self, request: Request<User>) -> Result<Response<User>, Status> {
        info!("validation request");

        let user = request.into_inner();
        self.validate(&user)?;

        Ok(Response::new(user))
    }

    #[tracing::instrument(skip(self))]
    async fn csrf(&self, request: Request<()>) -> Result<Response<()>, Status> {
        info!("csrf request");

        self.csrf.request_check(&request)?;

        self.csrf.response_used(Response::new(()))
    }

    type StreamingStream = ReceiverStream<Result<Echo, Status>>;

    #[tracing::instrument(skip(self))]
    async fn streaming(
        &self,
        request: Request<Echo>,
    ) -> Result<Response<Self::StreamingStream>, Status> {
        info!("streaming request");

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(Self::streaming_task(tx, request.into_inner()));

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
