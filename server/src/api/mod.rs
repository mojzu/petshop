//! API
//!
use crate::internal::*;
use petshop_proto::api::User;
use tokio::sync::broadcast;
use tonic::{Code, Request, Status};

mod petshop;
mod tfb;

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
    ) -> Result<Self, XErr> {
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
    pub async fn readiness(&self) -> Result<(), XErr> {
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
                _ => Err(Status::unauthenticated(ERROR_USER_AUTHENTICATION)),
            },
            _ => Err(Status::unauthenticated(ERROR_USER_AUTHENTICATION)),
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
