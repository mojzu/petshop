//! API
//!
use crate::internal::*;
use std::fmt;
use tokio::sync::broadcast;
use tonic::{Code, Status};

mod example;
mod petshop;
mod tfb;

/// API Server
#[derive(Clone)]
pub struct Api {
    pub shutdown: Arc<broadcast::Sender<bool>>,
    pub metrics: Arc<Metrics>,
    pub postgres: Arc<PostgresPool>,
    pub auth: Arc<Auth>,
    pub clients: Arc<Clients>,
    pub csrf: Arc<Csrf>,

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
    ) -> Result<Self, XErr> {
        let shutdown = Arc::new(shutdown_tx);
        let metrics = Arc::new(Metrics::from_config(config));
        let postgres = Arc::new(PostgresPool::from_config(config, metrics.clone())?);

        let auth = Arc::new(Auth::from_config(config, postgres.clone()));
        let clients = Arc::new(Clients::from_config(config)?);
        let csrf = Arc::new(Csrf::from_config(config, metrics.clone()));

        let mut tfb_handlebars = handlebars::Handlebars::new();
        tfb_handlebars
            .register_template_string("tfb_fortunes", TFB_FORTUNES_HTML)
            .expect("register template failed");

        Ok(Self {
            shutdown,
            metrics,
            postgres,
            auth,
            clients,
            csrf,
            tfb_handlebars: Arc::new(tfb_handlebars),
        })
    }

    /// Sends shutdown signal to stop application
    ///
    /// This lets the application trigger a graceful exit rather than panicking
    pub fn _shutdown(&self) {
        self.shutdown.send(true).expect("shutdown failed");
    }

    pub fn metrics(&self) -> Arc<Metrics> {
        self.metrics.clone()
    }

    pub fn csrf(&self) -> Arc<Csrf> {
        self.csrf.clone()
    }

    /// Returns an error if requests can not be served
    ///
    /// [More information on liveness/readiness probes](https://blog.colinbreck.com/kubernetes-liveness-and-readiness-probes-how-to-avoid-shooting-yourself-in-the-foot/)
    pub async fn readiness(&self) -> Result<(), XErr> {
        let postgres_ready = self.postgres.readiness().await;
        self.metrics.api_ready(postgres_ready.is_ok());
        postgres_ready?;
        Ok(())
    }

    /// Validates request using derived validate method, logs validation errors
    /// and uses status details to return serialised errors to client
    ///
    /// <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
    /// <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
    fn validate<T: validator::Validate>(&self, request: &T) -> Result<(), Status> {
        match request.validate() {
            Ok(_) => Ok(()),
            Err(err) => {
                self.metrics.validate_error_counter_inc();
                let anyhow_err: Error = err.clone().into();
                warn!("{:#}", anyhow_err);

                let status =
                    tonic_status_with_details(Code::InvalidArgument, ERROR_VALIDATION, err)?;

                Err(status)
            }
        }
    }
}

impl fmt::Debug for Api {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Api").finish()
    }
}
