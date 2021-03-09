//! # CSRF
//!
use crate::internal::*;
use cookie::SameSite;
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
    pub token_length: usize,
}

/// CSRF
pub struct Csrf {
    config: Option<CsrfConfig>,
}

impl Csrf {
    pub fn from_config(config: &Config) -> Self {
        Self {
            config: config.csrf.clone(),
        }
    }

    pub fn config(&self) -> Option<&CsrfConfig> {
        self.config.as_ref()
    }

    pub fn samesite_from_string(s: String) -> Result<SameSite> {
        match s.to_lowercase().as_ref() {
            "strict" => Ok(SameSite::Strict),
            "lax" => Ok(SameSite::Lax),
            "none" => Ok(SameSite::None),
            _ => Err(XError::config("csrf.cookie_samesite is invalid").into()),
        }
    }
}

impl fmt::Debug for Csrf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Csrf").finish()
    }
}
