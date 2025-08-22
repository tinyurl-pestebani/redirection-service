//! This module provides the `KeyGenerationService` trait and its implementations.
pub(crate) mod error;
mod grpc_generator;
pub(crate) mod layer;

use std::fmt::Debug;
use async_trait::async_trait;
use error::GeneratorError;


#[cfg(test)]
use mockall::automock;

/// A trait that string-based keys.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait KeyGenerationService: Debug + Send + Sync {
    /// Asynchronously generates a new key.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `String` representing the generated key,
    /// or a `GeneratorError` if key generation fails.
    async fn generate_key(&self) -> Result<String, GeneratorError>;
}
