//! This module provides a factory function for creating a `KeyGenerationService`.
use std::sync::Arc;
use anyhow::Result;
use crate::config::KeyGeneratorConfig;
use crate::key_generator::KeyGenerationService;
use crate::key_generator::grpc_generator::GRPCGenerator;


/// This function creates a new key generation service layer based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The configuration for the key generation service.
///
/// # Returns
///
/// A `Result` containing a new key generation service or an error.
pub async fn new_key_generation_service(config: &KeyGeneratorConfig) -> Result<Arc<dyn KeyGenerationService>> {
    match config {
        KeyGeneratorConfig::GRPCKeyGeneratorConfig(conf) => {
            let key_gen_service = GRPCGenerator::new(conf).await?;
            Ok(Arc::new(key_gen_service))
        },
        // Add other key generation configurations here
    }
}
