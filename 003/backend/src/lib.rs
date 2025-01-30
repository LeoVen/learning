pub mod api;
pub mod config;
pub mod services;

pub struct Dependencies {
    pub mongo: mongodb::Client,
}
