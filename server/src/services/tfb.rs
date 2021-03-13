use crate::internal::*;
use petshop_proto::api::tfb_server::Tfb;
use petshop_proto::api::{Echo, Fortune, Queries, World};
use petshop_proto::google::api::HttpBody;
use prost_types::{ListValue, Value};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl Tfb for Api {
    #[tracing::instrument(skip(self))]
    async fn tfb_json(&self, _request: Request<()>) -> Result<Response<Echo>, Status> {
        Ok(Response::new(Echo {
            message: "Hello, world!".to_string(),
        }))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_plaintext(&self, _request: Request<()>) -> Result<Response<HttpBody>, Status> {
        let body = HttpBody {
            content_type: "text/plain".to_string(),
            data: "Hello, world!".into(),
            extensions: vec![],
        };
        Ok(Response::new(body))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_db(&self, _request: Request<()>) -> Result<Response<World>, Status> {
        let world = self.postgres.db_world().await?;
        Ok(Response::new(world))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_queries(&self, request: Request<Queries>) -> Result<Response<ListValue>, Status> {
        let queries = request.into_inner().queries;
        let queries = if queries < 1 {
            1
        } else if queries > 500 {
            500
        } else {
            queries
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
    async fn tfb_fortunes(&self, _request: Request<()>) -> Result<Response<HttpBody>, Status> {
        let mut fortunes = self.postgres.db_fortunes().await?;
        fortunes.push(Fortune {
            id: 0,
            message: "Additional fortune added at request time.".to_string(),
        });
        fortunes.sort_by(|a, b| a.message.cmp(&b.message));

        let rows: Vec<serde_json::Value> = fortunes
            .into_iter()
            .map(|x| {
                json!({
                    "id": x.id,
                    "message": x.message,
                })
            })
            .collect();
        let template_data = json!({ "rows": rows });
        let template_render = self
            .tfb_handlebars
            .render("tfb_fortunes", &template_data)
            .unwrap();

        let body = HttpBody {
            content_type: "text/html; charset=UTF-8".to_string(),
            data: template_render.into(),
            extensions: vec![],
        };
        Ok(Response::new(body))
    }

    #[tracing::instrument(skip(self))]
    async fn tfb_updates(&self, request: Request<Queries>) -> Result<Response<ListValue>, Status> {
        let queries = request.into_inner().queries;
        let queries = if queries < 1 {
            1
        } else if queries > 500 {
            500
        } else {
            queries
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
