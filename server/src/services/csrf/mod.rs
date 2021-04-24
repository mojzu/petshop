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
const X_CSRF_ERROR: &str = "x-csrf-error";
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
            _ => Err(XErr::config("csrf.cookie_samesite is invalid").into()),
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

                // If error header is set, log it now
                if let Some(error) = request.metadata().get(X_CSRF_ERROR) {
                    if let Ok(error) = error.to_str() {
                        warn!("csrf check error: {}", error);
                    }
                }

                Err(tonic::Status::permission_denied(ERROR_CSRF_CHECK))
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
                let csrf_token = cookie_value(headers, config.cookie_name.as_str());

                // Get csrf token from header, this is set by the client
                let x_csrf_token = header_remove_value(headers, config.header_name.as_str());

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
                            let origin = header_get_value(headers, http::header::ORIGIN.as_str());
                            let referer = header_get_value(headers, http::header::REFERER.as_str());

                            // Match against the origin header (preferred), or fall back on the referer
                            // header, fail if neither are found
                            let origin_url = match (origin, referer) {
                                (Some(origin), _) => Url::from_str(&origin).ok(),
                                (_, Some(referer)) => Url::from_str(&referer).ok(),
                                _ => None,
                            };

                            if let Some(origin_url) = origin_url {
                                match_allow_origin(origin_url, &config.allow_origins)
                            } else {
                                // Blocking recommended if origin and referrer are not available
                                false
                            }
                        };

                        if allow_origin {
                            headers.insert(X_CSRF_MATCH, "1".parse().unwrap());
                        } else {
                            headers.insert(X_CSRF_ERROR, "origin not allowed".parse().unwrap());
                        }
                    } else {
                        headers.insert(X_CSRF_ERROR, "tokens do not match".parse().unwrap());
                    }
                } else {
                    headers.insert(X_CSRF_ERROR, "tokens not found".parse().unwrap());
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
                let csrf_used = header_remove_value(headers, X_CSRF_USED).is_some();

                // Always refresh the token if it has been used, else reuse
                // the token or generate one if it wasn't present
                let csrf_token = if csrf_used {
                    random_string(config.token_length)
                } else {
                    csrf_token.unwrap_or_else(|| random_string(config.token_length))
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
}

impl fmt::Debug for Csrf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Csrf").finish()
    }
}

/// Returns true if the origin matches one of the allowed origins
fn match_allow_origin(origin: Url, allow_origin: &[Url]) -> bool {
    for url in allow_origin {
        // Always match against scheme and domain
        let scheme_match = url.scheme() == origin.scheme();
        let domain_match = match (url.domain(), origin.domain()) {
            (Some(domain), Some(compare_domain)) => domain == compare_domain,
            _ => false,
        };
        // Match against port if it is present in the allowed origin
        let port_match = match url.port() {
            Some(port) => match origin.port() {
                Some(compare_port) => port == compare_port,
                _ => false,
            },
            None => true,
        };

        if scheme_match && domain_match && port_match {
            return true;
        }
    }

    false
}

/// Get cookie of name from headers and return value as string option
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

/// Get header of name from headers and return value as string option
fn header_get_value(headers: &HttpHeaders, header_name: &str) -> Option<String> {
    match headers.get(header_name) {
        Some(header) => match header.to_str() {
            Ok(value) => Some(value.into()),
            Err(_) => None,
        },
        None => None,
    }
}

/// Remove header of name from headers and return value as string option
fn header_remove_value(headers: &mut HttpHeaders, header_name: &str) -> Option<String> {
    match headers.remove(header_name) {
        Some(header) => match header.to_str() {
            Ok(value) => Some(value.into()),
            Err(_) => None,
        },
        None => None,
    }
}

/// Generate and return a random alphanumeric string of length
fn random_string(length: usize) -> String {
    use rand::Rng;
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_allow_origin_test() {
        let http_localhost = Url::from_str("http://localhost").unwrap();
        let allow_origin = vec![http_localhost];

        let http_localhost_1234 = Url::from_str("http://localhost:1234").unwrap();
        let http_localhost_4180 = Url::from_str("http://localhost:4180").unwrap();
        assert_eq!(match_allow_origin(http_localhost_1234, &allow_origin), true);
        assert_eq!(match_allow_origin(http_localhost_4180, &allow_origin), true);

        let http_foo = Url::from_str("http://foo").unwrap();
        let https_localhost = Url::from_str("https://localhost").unwrap();
        assert_eq!(match_allow_origin(http_foo, &allow_origin), false);
        assert_eq!(match_allow_origin(https_localhost, &allow_origin), false);

        let example_org = Url::from_str("http://example.org").unwrap();
        let allow_origin = vec![example_org.clone()];

        let attacker_com = Url::from_str("http://example.org.attacker.com").unwrap();
        assert_eq!(match_allow_origin(example_org, &allow_origin), true);
        assert_eq!(match_allow_origin(attacker_com, &allow_origin), false);
    }

    #[test]
    fn random_string_test() {
        let output = random_string(32);
        assert_eq!(output.len(), 32);
    }
}
