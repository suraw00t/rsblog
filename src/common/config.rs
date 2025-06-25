#![allow(unused)]
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env::var, net::Ipv4Addr, sync::Mutex};

static CONFIG_ENV: Lazy<Mutex<Option<Config>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub db_driver: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub db_options: Option<String>,
    pub workers: usize,
    pub address: Ipv4Addr,
    pub port: u16,
    pub prefix: String,
    pub rust_log: String,
    pub api_login_url: String,
    pub api_refresh_url: String,
}

impl Config {
    fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Config {
            db_driver: var("DB_DRIVER")
                .ok()
                .unwrap_or("mongodb".to_string())
                .to_lowercase(),
            db_host: var("DB_HOST").expect("env `DB_HOST` must be set"),
            db_port: var("DB_PORT").ok().unwrap_or("27017".to_string()),
            db_name: var("DB_NAME").expect("env `DB_NAME` must be set"),
            db_username: var("DB_USERNAME").ok(),
            db_password: var("DB_PASSWORD").ok(),
            db_options: var("DB_OPTIONS").ok(),
            workers: var("WORKERS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
            address: var("ADDRESS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Ipv4Addr::UNSPECIFIED),
            port: var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            prefix: var("PREFIX").ok().unwrap_or("".to_string()),
            rust_log: var("RUST_LOG").ok().unwrap_or("info".to_string()),
            api_login_url: var("API_LOGIN_URL")
                .ok()
                .unwrap_or("/api/v1/auth/login".to_string()),
            api_refresh_url: var("API_REFRESH_URL")
                .ok()
                .unwrap_or("/api/v1/auth/refresh".to_string()),
        }
    }

    pub fn init_from_env() {
        let config = Config::from_env();
        *CONFIG_ENV.lock().unwrap() = Some(config);
    }

    pub fn get_config() -> Config {
        CONFIG_ENV
            .lock()
            .unwrap()
            .clone()
            .expect("Config not initialized")
    }

    pub fn get_database_uri() -> String {
        let config = Config::get_config();
        let auth_part = match (config.db_username, config.db_password) {
            (Some(user), Some(pass)) => format!("{}:{}@", user, pass),
            _ => "".to_string(),
        };
        let options = config.db_options.clone().unwrap_or("".to_string());
        let uri = format!(
            "{}://{}{}:{}/{}{}",
            config.db_driver, auth_part, config.db_host, config.db_port, config.db_name, options
        );
        log::debug!("Database URI: {}", uri);
        uri
    }

    pub fn get_workers() -> usize {
        Config::get_config().workers
    }

    pub fn get_address() -> std::net::Ipv4Addr {
        Config::get_config().address
    }

    pub fn get_port() -> u16 {
        Config::get_config().port
    }

    pub fn get_prefix() -> String {
        Config::get_config().prefix
    }

    pub fn get_api_login_url() -> String {
        Config::get_prefix() + &Config::get_config().api_login_url
    }

    pub fn get_api_refresh_url() -> String {
        Config::get_prefix() + &Config::get_config().api_refresh_url
    }
}
