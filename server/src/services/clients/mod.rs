//! # Clients
use crate::internal::*;
use reqwest::Response;
use std::time::Duration;

/// Clients Configuration
#[derive(Debug, Clone)]
pub struct ClientsConfig {
    pub http_timeout_seconds: u64,
}

/// Clients
pub struct Clients {
    _config: ClientsConfig,
    http: reqwest::Client,
}

impl Clients {
    pub fn from_config(config: &Config) -> Result<Self, XErr> {
        let config = config.clients.clone();

        let http = reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(config.http_timeout_seconds))
            .use_rustls_tls()
            .build()?;

        Ok(Self {
            _config: config,
            http,
        })
    }

    /// Returns response from a GET request to url
    pub async fn get(&self, url: &str) -> Result<Response, XErr> {
        let req = self.http.get(url);
        let res = req.send().await?;
        Ok(res)
    }
}
