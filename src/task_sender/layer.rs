//! This module provides a factory function for creating a `TaskSender`.
use std::sync::Arc;
use anyhow::Result;
use crate::config::{RedirectionServiceConfig, TaskSender as TaskConfigSender};
use crate::task_sender::TaskSender;

/// This function creates a new task sender layer based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The configuration for the redirection service.
///
/// # Returns
///
/// A `Result` containing a new task sender or an error.
pub async fn new_task_sender(config: &RedirectionServiceConfig) -> Result<Arc<dyn TaskSender>> {
    match config.task_sender {
        TaskConfigSender::Nats(ref nats_sender_config) => {
            let nats_sender = crate::task_sender::nats::NatsTaskSender::new(nats_sender_config).await?;
            Ok(Arc::new(nats_sender))
        }
    }
}