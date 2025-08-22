//! This module provides the database layer for the application.
use std::fmt::Debug;
use async_trait::async_trait;
pub(crate) use crate::database::error::DatabaseError;

mod scylladb;
pub(crate) mod error;
pub(crate) mod layer;

#[cfg(test)]
use mockall::automock;

/// A trait that defines the operations for a database.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Database: Debug + Send + Sync {
    /// Retrieves the URL associated with a given key from the database.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The key to retrieve the URL for.
    ///
    /// # Returns
    ///
    /// A `Result` containing the URL or a `DatabaseError`.
    async fn get_key_url(&self, key_id: &String) -> Result<String, DatabaseError>;
    /// Inserts a new key-URL pair into the database.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The key to insert.
    /// * `url` - The URL to associate with the key.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the insertion was successful.
    async fn insert_key(&self, key_id: String, url: String) -> Result<(), DatabaseError>;
}
