#![allow(clippy::single_component_path_imports)]

/// Simple cli module
pub mod cli;
/// CRUD operations for db
pub mod crud;
/// Endpoints
pub mod endpoints;
/// Models of current db
pub mod models;
/// Autogenerated schema
pub mod schema;

use tracing;
use tracing_subscriber;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub fn init_logger() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let writer = std::io::stdout;
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(false)
        .flatten_event(true)
        .with_writer(writer);
    let subscriber = tracing_subscriber::Registry::default()
        .with(EnvFilter::from_default_env())
        .with(layer);
    tracing::subscriber::set_global_default(subscriber)
}