use mongodb::Client;

pub mod api;
pub mod config;
pub mod database;
pub mod model;
pub mod services;
pub mod tracing;

pub struct Dependencies {
    pub mongo: Client,
}
