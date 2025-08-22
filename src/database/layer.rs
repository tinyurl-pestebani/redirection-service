//! This module provides a factory function for creating a database layer.
use std::sync::Arc;
use anyhow::Result;
use crate::config::{DBConfig, RedirectionServiceConfig};
use crate::database::Database;
use crate::database::scylladb::ScyllaDB;


/// This function creates a new database layer based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The configuration for the redirection service.
///
/// # Returns
///
/// A `Result` containing a new database layer or an error.
pub async fn new_db_layer(config: &RedirectionServiceConfig) -> Result<Arc<dyn Database>> {
    // This function creates a new database layer.
    // It returns an Arc<dyn Database> which is a trait object.
    match config.db_config {
        DBConfig::ScyllaDB(ref config) => {
            let db = ScyllaDB::new(config).await?;
            Ok(Arc::new(db))
        },
    }
}
