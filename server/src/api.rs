//! # API
//!
use tonic::{Request, Response, Status};

use petshop_proto::petshop_server::Petshop;
use petshop_proto::{
    Category, FindByStatus, FindByTag, HttpBody, Pet, Pets, Status as PetStatus, Tag,
};

use crate::internal::*;

/// API Server
#[derive(Clone)]
pub struct Api {
    metrics: Arc<Metrics>,
}

impl Api {
    pub fn from_config(config: &Config) -> Self {
        Self {
            metrics: Arc::new(Metrics::from_config(config)),
        }
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    /// Runs before request when using `api_request!` macro
    fn pre_request<T>(&self, _req: &Request<T>) -> SystemTime {
        let request_start = SystemTime::now();
        self.metrics().api_counter_inc();
        request_start
    }

    /// Runs after request when using `api_request!` macro
    fn post_request<T>(&self, request_start: SystemTime, response: T) -> T {
        self.metrics.api_latency_record(request_start);
        response
    }
}

/// Utility macro to make running common code before and after a request easier
///
/// TODO: Could api request wrapping be done better/more intuitively with middleware?
macro_rules! api_request {
    ($api:expr, $req:expr, $e:expr) => {{
        let api_request_start = $api.pre_request($req);
        let api_response = $e.await;
        $api.post_request(api_request_start, api_response)
    }};
}

#[tonic::async_trait]
impl Petshop for Api {
    async fn http_body(&self, request: Request<HttpBody>) -> Result<Response<HttpBody>, Status> {
        api_request!(self, &request, async {
            info!("http_body request: {:?}", request);
            Ok(Response::new(request.into_inner()))
        })
    }

    async fn pet_post(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        api_request!(self, &request, async {
            info!("pet_post request: {:?}", request);
            Ok(Response::new(request.into_inner()))
        })
    }

    async fn pet_put(&self, request: Request<Pet>) -> Result<Response<Pet>, Status> {
        api_request!(self, &request, async {
            info!("pet_put request: {:?}", request);
            Ok(Response::new(request.into_inner()))
        })
    }

    async fn pet_find_by_status(
        &self,
        request: Request<FindByStatus>,
    ) -> Result<Response<Pets>, Status> {
        api_request!(self, &request, async {
            info!("pet_find_by_status request: {:?}", request);
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
        })
    }

    async fn pet_find_by_tag(&self, request: Request<FindByTag>) -> Result<Response<Pets>, Status> {
        api_request!(self, &request, async {
            info!("pet_find_by_tag request: {:?}", request);
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
        })
    }
}
