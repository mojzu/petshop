//! # CSRF Service
//!
use crate::internal::*;
use cookie::Cookie;
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService};
use tower::Service;

/// Service interceptor to manage csrf tokens
#[derive(Debug, Clone)]
pub struct CsrfService<S> {
    csrf: Arc<Csrf>,
    inner: S,
}

impl<S> CsrfService<S> {
    pub fn wrap(csrf: Arc<Csrf>, api: S) -> Self {
        Self { csrf, inner: api }
    }
}

impl<S> Service<HyperRequest<Body>> for CsrfService<S>
where
    S: Service<HyperRequest<Body>, Response = HyperResponse<BoxBody>>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: HyperRequest<Body>) -> Self::Future {
        let mut svc = self.inner.clone();
        let csrf = self.csrf.clone();

        Box::pin(async move {
            match csrf.config() {
                Some(config) => {
                    let xsrf_token = xsrf_token_cookie_value(config, &req);
                    let x_xsrf_token = x_xsrf_token_header_value(config, &req);
                    // TODO: Implement verification here or allow tonic handlers to check
                    // If there is a cookie, verify it with key/timestamp (with user as well?)
                    // If cookie verified and there is a header, verify values match
                    // If values match, set header to indicate csrf success for tonic handler?
                    debug!(
                        "xsrf_token = {:?} / x_xsrf_token = {:?}",
                        xsrf_token, x_xsrf_token
                    );

                    let res = svc.call(req).await?;
                    let res = xsrf_handler(config, xsrf_token, res).await;
                    Ok(res)
                }
                // Service does nothing if config not provided
                None => svc.call(req).await,
            }
        })
    }
}

impl<S: NamedService> NamedService for CsrfService<S> {
    const NAME: &'static str = S::NAME;
}

async fn xsrf_handler(
    config: &CsrfConfig,
    cookie: Option<String>,
    mut res: HyperResponse<BoxBody>,
) -> HyperResponse<BoxBody> {
    let status_is_ok = res.status() == http::StatusCode::OK;
    let code_is_ok = grpc_status_header_code(&res) == tonic::Code::Ok;

    if status_is_ok && code_is_ok {
        // TODO: Some method of timer/rotation/invalidation here, see example
        // <https://github.com/django/django/blob/main/django/middleware/csrf.py>
        let cookie = cookie.unwrap_or_else(|| random_string(config));

        let cookie = Cookie::build(config.cookie_name.as_str(), cookie)
            .domain(config.cookie_domain.as_str())
            .path(config.cookie_path.as_str())
            .secure(config.cookie_secure)
            .http_only(false)
            .same_site(config.cookie_samesite)
            .max_age(time::Duration::minutes(config.cookie_max_age_minutes))
            .finish();

        res.headers_mut().append(
            http::header::SET_COOKIE,
            cookie.to_string().parse().unwrap(),
        );
    }

    res
}

fn xsrf_token_cookie_value(config: &CsrfConfig, req: &HyperRequest<Body>) -> Option<String> {
    match req.headers().get(http::header::COOKIE) {
        Some(header) => match header.to_str() {
            Ok(header) => {
                let cookies: Vec<&str> = header.split_whitespace().collect();
                for cookie in cookies {
                    if let Ok(cookie) = Cookie::parse(cookie) {
                        if cookie.name().to_lowercase()
                            == config.cookie_name.as_str().to_lowercase()
                        {
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

fn x_xsrf_token_header_value(config: &CsrfConfig, req: &HyperRequest<Body>) -> Option<String> {
    match req.headers().get(config.header_name.as_str()) {
        Some(header) => match header.to_str() {
            Ok(value) => Some(value.into()),
            Err(_) => None,
        },
        None => None,
    }
}

/// Returns grpc-status code from response headers, if header is not present assumed to be 0
fn grpc_status_header_code(res: &HyperResponse<BoxBody>) -> tonic::Code {
    match res.headers().get("grpc-status") {
        Some(header) => match header.to_str() {
            Ok(value) => {
                match value.parse::<i32>() {
                    Ok(value) => {
                        tonic::Code::from_i32(value)
                    },
                    Err(_) => tonic::Code::Unknown,
                }
            },
            Err(_) => tonic::Code::Unknown,
        },
        None => tonic::Code::Ok,
    }
}

fn random_string(config: &CsrfConfig) -> String {
    use rand::Rng;
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(config.token_length)
        .map(char::from)
        .collect()
}
