//! This is the main entry point for the redirection service.
//! It sets up the database, task sender, key generator, and the Axum server.
use axum::Router;
use axum::routing::{post, get};

use anyhow::Result;

use rust_otel_setup::otel::OpenTelemetryObject;
use rust_otel_setup::config as otel_config;
use tracing::log::{debug, info};

mod database;
mod app;
mod task_sender;
mod config;
mod key_generator;

use app::AppState;
use app::handlers::create_url;
use crate::app::handlers::{get_url, ROUTE_CREATE_URL, ROUTE_GET_URL};
use crate::config::RedirectionServiceConfig;


/// The main entry point for the application.
#[tokio::main]
async fn main() -> Result<()> {
    let config = RedirectionServiceConfig::from_env()?;
    debug!("Connecting to database");
    let db_layer = database::layer::new_db_layer(&config).await?;
    debug!("Connected to database");
    debug!("Connecting to task queue sender");
    let task_sender = task_sender::layer::new_task_sender(&config).await?;
    debug!("Connected to task queue sender");
    debug!("Starting key generator");
    let key_generator = key_generator::layer::new_key_generation_service(&config.key_generator).await?;
    debug!("Key generator started");
    debug!("Starting OpenTelemetry");

    let otel_object = OpenTelemetryObject::new(&otel_config::LogConfig::from_env()?, &otel_config::TraceConfig::from_env()?, "redirection-service".into()).await?;
    debug!("OpenTelemetry started");
    
    let app_state = AppState::new(db_layer, task_sender, key_generator).await?;
    let app = Router::new()
        .route(ROUTE_CREATE_URL, post(create_url))
        .route(ROUTE_GET_URL, get(get_url))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("[::]:{}", config.port))
        .await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async move { 
            tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            otel_object.stop().unwrap();
        })
        .await?;
    Ok(())
}
