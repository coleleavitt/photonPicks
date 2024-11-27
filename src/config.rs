use serde::Deserialize;
use std::path::PathBuf;
use config::{Config, ConfigError, File, FileFormat};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    #[allow(dead_code)]
    pub websocket: WebSocketConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WebSocketConfig {
    pub max_connections: usize,
    pub timeout_seconds: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = PathBuf::from("config");
        let config = Config::builder()
            .add_source(File::new(
                config_dir.join("default.toml").to_str().unwrap(),
                FileFormat::Toml,
            ))
            .add_source(File::new(
                config_dir.join("local.toml").to_str().unwrap(),
                FileFormat::Toml,
            ).required(false))
            .build()?;

        config.try_deserialize()
    }
}