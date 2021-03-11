//! # CSRF
//!
//! Based on advice at the following link:
//! <https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html>
//!
//! - Double submit cookie strategy (works with axios/Angular)
//! - SameSite cookie attribute (defaults to strict)
//! - Optional origin verification
//!
use crate::internal::*;
use cookie::{Cookie, SameSite};
use std::fmt;

pub use service::CsrfService;

mod service;

/// CSRF Configuration
#[derive(Debug, Clone)]
pub struct CsrfConfig {
    pub cookie_name: String,
    pub cookie_domain: String,
    pub cookie_path: String,
    pub cookie_secure: bool,
    pub cookie_samesite: SameSite,
    pub cookie_max_age_minutes: i64,
    pub header_name: String,
    pub allow_origins: Vec<Url>,
    pub token_length: usize,
}

/// CSRF
pub struct Csrf {
    config: Option<CsrfConfig>,
    metrics: Arc<Metrics>,
}

const X_CSRF_MATCH: &str = "x-csrf-match";
const X_CSRF_USED: &str = "x-csrf-used";

impl Csrf {
    pub fn from_config(config: &Config, metrics: Arc<Metrics>) -> Self {
        Self {
            metrics,
            config: config.csrf.clone(),
        }
    }

    pub fn samesite_from_string(s: String) -> Result<SameSite> {
        match s.to_lowercase().as_ref() {
            "strict" => Ok(SameSite::Strict),
            "lax" => Ok(SameSite::Lax),
            "none" => Ok(SameSite::None),
            _ => Err(XError::config("csrf.cookie_samesite is invalid").into()),
        }
    }

    /// Used in tonic request handlers to check CSRF match
    pub fn request_check(&self, request: &tonic::Request<()>) -> Result<(), tonic::Status> {
        // If configuration is None, csrf is disabled
        if self.config.is_some() {
            // If match header is set on request then tokens matched
            if request.metadata().get(X_CSRF_MATCH).is_some() {
                Ok(())
            } else {
                self.metrics.csrf_error_counter_inc();
                // FIXME: Would it be worth using an x-csrf-error header set by
                // the request handler to log as an error here?

                Err(tonic::Status::permission_denied("csrf check failed"))
            }
        } else {
            Ok(())
        }
    }

    /// Used in tonic request handlers to mark CSRF as used in response metadata
    pub fn response_used<T>(
        &self,
        mut response: tonic::Response<T>,
    ) -> Result<tonic::Response<T>, tonic::Status> {
        // If configuration is None, csrf is disabled
        if self.config.is_some() {
            // Set used header on response to refresh token
            response
                .metadata_mut()
                .insert(X_CSRF_USED, "1".parse().unwrap());
        }
        Ok(response)
    }

    /// Used in service to check request headers for CSRF match
    pub fn service_request_handler(&self, headers: &mut HttpHeaders) -> Option<String> {
        // If configuration is None, csrf is disabled
        match self.config.as_ref() {
            Some(config) => {
                // Get csrf token from cookie, this is set by the server
                // on successful responses and refresh after one use
                //
                // This does require some kind of initialisation by the client to
                // make a first request that does not require csrf verification
                let csrf_token = Self::cookie_value(headers, config.cookie_name.as_str());

                // Get csrf token from header, this is set by the client
                let x_csrf_token = Self::header_remove_value(headers, config.header_name.as_str());

                // Check if cookie and header csrf tokens match, if they do
                // set a header on the request which can be checked in
                // the tonic request handler
                //
                // FIXME: Use HMAC based token pattern here?
                if let (Some(csrf_token), Some(x_csrf_token)) =
                    (csrf_token.as_ref(), x_csrf_token.as_ref())
                {
                    if csrf_token == x_csrf_token {
                        let allow_origin = if config.allow_origins.is_empty() {
                            true
                        } else {
                            // Check the source origin if allow_origin is not empty
                            // <https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html#identifying-source-origin-via-originreferer-header>
                            Self::match_allow_origin(headers, &config.allow_origins)
                        };

                        if allow_origin {
                            headers.append(X_CSRF_MATCH, "1".parse().unwrap());
                        }
                    }
                }

                // Return the csrf token, it will be reused on the
                // response unless used by the tonic request handler
                csrf_token
            }
            None => None,
        }
    }

    /// Used in service to check response headers for CSRF used
    pub fn service_response_handler(
        &self,
        csrf_token: Option<String>,
        status: HttpStatus,
        headers: &mut HttpHeaders,
    ) {
        // If configuration is None, csrf is disabled
        if let Some(config) = self.config.as_ref() {
            // Check the HTTP/gRPC status is OK before setting cookie
            let status_is_ok = status == HttpStatus::OK;
            let code_is_ok = http_headers_grpc_status(headers) == tonic::Code::Ok;

            if status_is_ok && code_is_ok {
                // Tonic request handler indicates that the token has been used with header
                let csrf_used = Self::header_remove_value(headers, X_CSRF_USED).is_some();

                // Always refresh the token if it has been used, else reuse
                // the token or generate one if it wasn't present
                let csrf_token = if csrf_used {
                    Self::random_string(config.token_length)
                } else {
                    csrf_token.unwrap_or_else(|| Self::random_string(config.token_length))
                };

                // Create cookie and set on response
                let cookie = Cookie::build(config.cookie_name.as_str(), csrf_token)
                    .domain(config.cookie_domain.as_str())
                    .path(config.cookie_path.as_str())
                    .secure(config.cookie_secure)
                    .http_only(false)
                    .same_site(config.cookie_samesite)
                    .max_age(time::Duration::minutes(config.cookie_max_age_minutes))
                    .finish();

                headers.append(
                    http::header::SET_COOKIE,
                    cookie.to_string().parse().unwrap(),
                );
            }
        }
    }

    fn match_allow_origin(headers: &HttpHeaders, allow_origin: &[Url]) -> bool {
        let origin = Self::header_get_value(headers, http::header::ORIGIN.as_str());
        let referer = Self::header_get_value(headers, http::header::REFERER.as_str());

        // Match against the origin header (preferred), or fall back on the referer
        // header, fail if neither are found
        let compare_url = match (origin, referer) {
            (Some(origin), _) => Url::from_str(&origin).ok(),
            (_, Some(referer)) => Url::from_str(&referer).ok(),
            _ => None,
        };

        // Compare URL against allowed origins
        if let Some(compare_url) = compare_url {
            for url in allow_origin {
                // Always match against scheme and domain
                let scheme_match = url.scheme() == compare_url.scheme();
                let domain_match = match (url.domain(), compare_url.domain()) {
                    (Some(domain), Some(compare_domain)) => domain == compare_domain,
                    _ => false,
                };
                // Match against port if it is present in the allowed origin
                let port_match = match url.port() {
                    Some(port) => match compare_url.port() {
                        Some(compare_port) => port == compare_port,
                        _ => false,
                    },
                    None => true,
                };

                if scheme_match && domain_match && port_match {
                    return true;
                }
            }
        }

        false
    }

    fn cookie_value(headers: &HttpHeaders, cookie_name: &str) -> Option<String> {
        match headers.get(http::header::COOKIE) {
            Some(header) => match header.to_str() {
                Ok(header) => {
                    let cookies: Vec<&str> = header.split_whitespace().collect();
                    for cookie in cookies {
                        if let Ok(cookie) = Cookie::parse(cookie) {
                            if cookie.name().to_lowercase() == cookie_name.to_lowercase() {
                                return Some(cookie.value().into());
                            }
                        }
                    }
                    None
                }
                Err(_) => None,
            },
            None => None,
        }
    }

    fn header_get_value(headers: &HttpHeaders, header_name: &str) -> Option<String> {
        match headers.get(header_name) {
            Some(header) => match header.to_str() {
                Ok(value) => Some(value.into()),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn header_remove_value(headers: &mut HttpHeaders, header_name: &str) -> Option<String> {
        match headers.remove(header_name) {
            Some(header) => match header.to_str() {
                Ok(value) => Some(value.into()),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn random_string(length: usize) -> String {
        use rand::Rng;
        let rng = rand::thread_rng();
        rng.sample_iter(&rand::distributions::Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}

impl fmt::Debug for Csrf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Csrf").finish()
    }
}
