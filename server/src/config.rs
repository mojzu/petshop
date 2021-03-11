//! # Configuration
//!
use crate::internal::*;
use std::fmt;
use std::net::SocketAddr;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::EnvFilter;

/// Configuration
///
/// Final server configuration goes here, derived from the `ConfigLoad` struct.
/// `TryFrom<ConfigLoad>` applies defaults/validation, etc.
#[derive(Debug, Clone)]
pub struct Config {
    pub tracing_json: bool,
    pub api_addr: SocketAddr,
    pub internal_addr: SocketAddr,
    pub metrics_name: String,
    pub csrf: Option<CsrfConfig>,
    pub postgres: deadpool_postgres::Config,
}

#[derive(Debug, Clone, Deserialize)]
struct CsrfConfigLoad {
    cookie_name: Option<String>,
    cookie_domain: Option<String>,
    cookie_path: Option<String>,
    cookie_secure: Option<bool>,
    cookie_samesite: Option<String>,
    cookie_max_age_minutes: Option<i64>,
    header_name: Option<String>,
    token_length: Option<usize>,
}

/// Parsed Configuration
///
/// Parsed server configuration goes here, these fields are optional to allow
/// failing gracefully in case a value is undefined.
/// Values can be loaded from environment variables with a `CONFIG_` prefix
/// or from a configuration file.
#[derive(Debug, Clone, Deserialize)]
struct ConfigLoad {
    tracing_json: Option<bool>,
    api_host: Option<String>,
    api_port: Option<u16>,
    internal_host: Option<String>,
    internal_port: Option<u16>,
    metrics_name: Option<String>,
    csrf: Option<CsrfConfigLoad>,
    postgres: Option<deadpool_postgres::Config>,
}

impl TryFrom<ConfigLoad> for Config {
    type Error = Error;

    fn try_from(value: ConfigLoad) -> Result<Config> {
        // This performs the conversion from ConfigLoad to Config
        // Warnings/other information can be printed here to inform users about options
        // Can't use log macros here because tracing has not been initialised yet
        let tracing_json = Config::opt_or_default("tracing_json", value.tracing_json, false);
        let api_host = Config::opt_or_default("api_host", value.api_host, "127.0.0.1".to_string());
        let api_port = Config::opt_or_default("api_port", value.api_port, 5000);
        let api_addr: SocketAddr = format!("{}:{}", api_host, api_port).parse()?;
        let internal_host = Config::opt_or_default(
            "internal_host",
            value.internal_host,
            "127.0.0.1".to_string(),
        );
        let internal_port = Config::opt_or_default("internal_port", value.internal_port, 5501);
        let internal_addr: SocketAddr = format!("{}:{}", internal_host, internal_port).parse()?;
        let metrics_name =
            Config::opt_or_default("metrics_name", value.metrics_name, NAME.to_string());

        let csrf = if let Some(csrf) = value.csrf {
            let cookie_name = Config::opt_or_default(
                "csrf.cookie_name",
                csrf.cookie_name,
                "XSRF-TOKEN".to_string(),
            );
            let cookie_domain = Config::opt_or_default(
                "csrf.cookie_domain",
                csrf.cookie_domain,
                "localhost".to_string(),
            );
            let cookie_path =
                Config::opt_or_default("csrf.cookie_path", csrf.cookie_path, "/".to_string());
            let cookie_secure =
                Config::opt_or_default("csrf.cookie_secure", csrf.cookie_secure, true);
            let cookie_samesite = Config::opt_or_default(
                "csrf.cookie_samesite",
                csrf.cookie_samesite,
                "strict".to_string(),
            );
            let cookie_samesite = Csrf::samesite_from_string(cookie_samesite)?;
            let cookie_max_age_minutes = Config::opt_or_default(
                "csrf.cookie_max_age_minutes",
                csrf.cookie_max_age_minutes,
                1440,
            );
            let header_name = Config::opt_or_default(
                "csrf.header_name",
                csrf.header_name,
                "X-XSRF-TOKEN".to_string(),
            );
            let token_length = Config::opt_or_default("csrf.token_length", csrf.token_length, 32);
            Some(CsrfConfig {
                cookie_name,
                cookie_domain,
                cookie_path,
                cookie_secure,
                cookie_samesite,
                cookie_max_age_minutes,
                header_name,
                token_length,
            })
        } else {
            println!("Config: csrf is not configured, defaulting to disabled");
            None
        };

        let mut postgres = if let Some(postgres) = value.postgres {
            postgres
        } else {
            return Err(XError::config("postgres is not configured").into());
        };
        if postgres.application_name.is_none() {
            let application_name = USER_AGENT.to_string();
            postgres.application_name = Some(Config::opt_or_default(
                "postgres.application_name",
                None,
                application_name,
            ));
        }
        if postgres.connect_timeout.is_none() {
            let connect_timeout = std::time::Duration::from_secs(5);
            postgres.connect_timeout = Some(Config::opt_or_default(
                "postgres.connect_timeout",
                None,
                connect_timeout,
            ));
        }

        Ok(Config {
            tracing_json,
            api_addr,
            internal_addr,
            metrics_name,
            csrf,
            postgres,
        })
    }
}

impl Config {
    /// Parse configuration from optional file path and environment variables using
    /// default `CONFIG_` prefix
    ///
    /// Environment variables overwrite values defined in configuration file
    pub fn load(file_path: Option<&str>) -> Result<Self> {
        Self::load_with_prefix("CONFIG", file_path)
    }

    /// Parse configuration from optional file path and environment variables with prefix
    pub fn load_with_prefix(prefix: &str, file_path: Option<&str>) -> Result<Self> {
        let mut cfg = ::config::Config::new();

        if let Some(file_path) = file_path {
            cfg.merge(::config::File::with_name(file_path))?;
        }
        cfg.merge(::config::Environment::with_prefix(prefix).separator("__"))?;

        let load: ConfigLoad = cfg.try_into()?;
        load.try_into()
    }

    /// Initialise panic and log output to stderr using tracing and configuration values
    ///
    /// <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
    /// <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
    /// <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#data-to-exclude>
    pub fn init_panic_and_tracing(&self) {
        if self.tracing_json {
            Self::init_panic_json();
        }

        let builder = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_timer(ChronoUtc::default())
            .with_writer(std::io::stderr);
        if self.tracing_json {
            builder.json().init();
        } else {
            builder.pretty().init();
        }

        debug!("{:?}", self);
    }

    fn init_panic_json() {
        std::panic::set_hook(Box::new(|info| {
            let location = info.location().expect("panic location failed");
            let output = serde_json::to_string(&json!({
                "timestamp": Utc::now().to_rfc3339(),
                "level": tracing::Level::ERROR.to_string(),
                "fields": {
                    "message": format!("{}", info),
                    "file": format!("{}:{}:{}", location.file(), location.line(), location.column()),
                },
                "target": NAME,
                "version": VERSION,
            }))
                .expect("panic_json failed");
            eprintln!("{}", output);
        }));
    }

    fn opt_or_default<T: fmt::Debug>(name: &str, value: Option<T>, default_value: T) -> T {
        if let Some(value) = value {
            value
        } else {
            println!(
                "Config: {} is not configured, defaulting to {:?}",
                name, default_value
            );
            default_value
        }
    }
}
