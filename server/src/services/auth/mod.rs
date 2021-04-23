//! # Auth
use crate::internal::*;
use petshop_proto::api::User;
use tonic::{Code, Request, Status};

/// Auth
pub struct Auth {
    _postgres: Arc<PostgresPool>,
}

impl Auth {
    pub fn from_config(_config: &Config, postgres: Arc<PostgresPool>) -> Self {
        Self {
            _postgres: postgres,
        }
    }

    /// Parses request metadata to extract authenticated user (works with auth example)
    pub fn user_interceptor(request: &Request<()>) -> Result<User, Status> {
        let email = request.metadata().get("x-auth-request-email");
        let user = request.metadata().get("x-auth-request-user");
        match (email, user) {
            (Some(email), Some(user)) => match (email.to_str(), user.to_str()) {
                (Ok(email), Ok(user)) => Ok(User {
                    email: email.to_string(),
                    name: user.to_string(),
                }),
                _ => Err(Status::unauthenticated(ERROR_AUTHENTICATION)),
            },
            _ => Err(Status::unauthenticated(ERROR_AUTHENTICATION)),
        }
    }

    /// Parses request metadata to extract authorization header (works with auth example)
    ///
    /// FIXME: This is a placeholder function that accepts any authorization header as valid to demonstrate
    /// supporting authentication via a header (e.g. for api consumers)
    pub fn api_interceptor(request: &Request<()>) -> Result<User, Status> {
        let auth = request.metadata().get("authorization");
        match auth {
            Some(auth) => match auth.to_str() {
                Ok(auth) => Ok(User {
                    email: "apiconsumer@petshop.com".to_string(),
                    name: auth.to_string(),
                }),
                _ => Err(Status::unauthenticated(ERROR_AUTHENTICATION)),
            },
            _ => Err(Status::unauthenticated(ERROR_AUTHENTICATION)),
        }
    }

    /// Wraps user interceptor function in case config is useful here later on
    pub async fn user(&self, request: &Request<()>) -> Result<User, Status> {
        Self::user_interceptor(request)
    }

    /// Wraps api interceptor function in case config is useful here later on
    pub async fn api(&self, request: &Request<()>) -> Result<User, Status> {
        Self::api_interceptor(request)
    }

    /// Parses request metadata to return authenticated user, which may be provided by oauth2-proxy
    /// headers or by the authorization header
    ///
    /// In the auth example, this is made functional by adding an envoy listener that does not use the
    /// ext_authz filter, so requests are still passed upstream where they can be checked by this function
    ///
    /// This example assumes that the application is going to manage/verify its own API keys and that
    /// all private endpoints will call this function
    ///
    /// <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#api-keys>
    pub async fn api_or_user(&self, request: &Request<()>) -> Result<User, Status> {
        match self.api(request).await {
            Ok(user) => Ok(user),
            Err(err) => match err.code() {
                Code::Unauthenticated => self.user(request).await,
                _ => Err(err),
            },
        }
    }
}
