//! # Configuration
//!
use std::fmt;
use std::net::SocketAddr;

use crate::internal::*;

/// Configuration
///
/// Final server configuration goes here, derived from the `ConfigLoad` struct.
/// `TryFrom<ConfigLoad>` applies defaults/validation, etc.
#[derive(Debug, Clone)]
pub struct Config {
    pub panic_json: bool,
    pub log_json: bool,
    pub api_addr: SocketAddr,
    pub internal_addr: SocketAddr,
}

/// Parsed Configuration
///
/// Parsed server configuration goes here, these fields are optional to allow
/// failing gracefully in case a value is undefined.
/// Values can be loaded from environment variables with a `CONFIG_` prefix
/// or from a configuration file.
#[derive(Debug, Clone, Deserialize)]
struct ConfigLoad {
    panic_json: Option<bool>,
    log_json: Option<bool>,
    api_host: Option<String>,
    api_port: Option<u16>,
    internal_host: Option<String>,
    internal_port: Option<u16>,
}

impl TryFrom<ConfigLoad> for Config {
    type Error = Error;

    fn try_from(value: ConfigLoad) -> Result<Config> {
        // This performs the conversion from ConfigLoad to Config
        // Warnings/other information can be printed here to inform users about options
        // Can't use log macros here because env_logger has not been initialised yet
        let panic_json = Config::opt_or_default("panic_json", value.panic_json, false);
        let log_json = Config::opt_or_default("log_json", value.log_json, false);
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

        Ok(Config {
            panic_json,
            log_json,
            api_addr,
            internal_addr,
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

    /// Initialise panic and log output to stderr using env_logger and configuration values
    pub fn init_panic_and_log(&self) {
        if self.panic_json {
            Self::init_panic_json();
        }
        if self.log_json {
            Self::init_log_json();
        } else {
            env_logger::init();
        }
        debug!("{:?}", self);
    }

    fn init_panic_json() {
        std::panic::set_hook(Box::new(|info| {
            let location = info.location().unwrap();
            let message = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                },
            };

            let output = serde_json::to_string(&json!({
                "time": Utc::now().to_rfc3339(),
                "level": log::Level::Error.to_string(),
                "file": format!("{}:{}:{}", location.file(), location.line(), location.column()),
                "panic": true,
                "message": message,
                "name": NAME,
                "version": VERSION,
            }))
            .expect("panic_json failure");
            eprintln!("{}", output);
        }));
    }

    fn init_log_json() {
        use std::io::Write;

        let mut builder = env_logger::Builder::from_default_env();
        builder.format(move |buf, record| {
            let file = record.file();
            let line = record.line();
            let message = record.args();

            let output = serde_json::to_string(&json!({
                "time": Utc::now().to_rfc3339(),
                "level": record.level().to_string(),
                "target": record.target(),
                "module_path": record.module_path(),
                "file": format!("{}:{}", file.unwrap_or("none"), line.unwrap_or(0)),
                "message": message,
                "name": NAME,
                "version": VERSION,
            }))
            .expect("log_json failure");

            writeln!(buf, "{}", output)
        });

        builder.init();
    }

    fn opt_or_default<T: fmt::Display>(name: &str, value: Option<T>, default_value: T) -> T {
        if let Some(value) = value {
            value
        } else {
            println!(
                "config: {} is not configured, defaulting to {}",
                name, default_value
            );
            default_value
        }
    }
}
