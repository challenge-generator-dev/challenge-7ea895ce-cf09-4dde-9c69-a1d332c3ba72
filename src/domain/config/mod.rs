use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;

        let server_port = env::var("SERVER_PORT")
            .map_err(|_| ConfigError::MissingEnvVar("SERVER_PORT".to_string()))
            .and_then(|port| {
                port.parse::<u16>()
                    .map_err(|_| ConfigError::InvalidEnvVar("SERVER_PORT".to_string()))
            })?;

        Ok(Config {
            database_url,
            server_port,
        })
    }
}