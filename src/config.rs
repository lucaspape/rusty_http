use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RustyHTTPConfig {
    pub servers: Vec<ServerConfig>
}

impl RustyHTTPConfig {
    pub fn read(filename: &str) -> RustyHTTPConfig {
        let json = fs::read_to_string(filename).unwrap();

        let config = serde_json::from_str(json.as_str()).unwrap();

        return config;
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind: String,
    pub default_host: HostConfig,
    pub hosts: Vec<HostConfig>
}

#[derive(Serialize, Deserialize)]
pub struct HostConfig {
    pub server_name: String,
    pub locations: Vec<LocationConfig>
}

#[derive(Serialize, Deserialize)]
pub struct LocationConfig {
    pub path: String,
    pub extension: Option<String>,
    pub root: String,
    pub index: Option<bool>
}