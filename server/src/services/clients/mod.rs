//! # Clients
use crate::internal::*;
use http::{header, Request, Response};
use hyper::{client, Body};
use hyper_rustls::HttpsConnector;

/// Clients
pub struct Clients {
    http: client::Client<HttpsConnector<client::HttpConnector>, Body>,
}

impl Clients {
    pub fn from_config(_config: &Config) -> Self {
        // TODO: Configurable client options from config
        let http = client::Client::builder().build(HttpsConnector::with_native_roots());

        Self { http }
    }

    /// Returns response from a GET request to uri
    pub async fn get(&self, uri: &str) -> Result<Response<Body>, XErr> {
        // TODO: Cleanup this, add other methods, utility functions for json, etc?
        let mut req = Request::get(uri);
        if let Some(headers) = req.headers_mut() {
            headers.insert(header::USER_AGENT, USER_AGENT.parse().unwrap());
        }
        let req = req.body(Body::empty())?;

        let res = self.http.request(req).await?;
        Ok(res)
    }
}
