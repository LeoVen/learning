use std::fs;

use anyhow::Context;
use serde::Deserialize;
use toml;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Environment {
    #[serde(rename(deserialize = "dev"))]
    DEV,
    STAGE,
    PROD,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub env: Environment,
    pub proxy: ProxyConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProxyConfig {
    pub name: String,
    pub port: u16,
    pub upstreams: Vec<String>,
}

impl Config {
    pub fn load(filename: &str) -> anyhow::Result<Self> {
        let contents =
            fs::read_to_string(filename).with_context(|| format!("failed to read {}", filename))?;
        toml::from_str(&contents).context("failed to parse toml config")
    }
}
