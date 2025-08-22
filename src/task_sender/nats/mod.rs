//! This module contains the NATS implementation of the `TaskSenderBytes` trait.
use async_trait::async_trait;
use async_nats::jetstream::{self, context::Context};
use bytes::Bytes;
use anyhow::Result;
use crate::config::NatsConfig;
use crate::task_sender::TaskSenderBytes;

/// This struct is a NATS client for sending tasks.
#[derive(Clone, Debug)]
pub struct NatsTaskSender {
    ctx: Context,
    subject: String,
}


impl NatsTaskSender {
    /// Creates a new `NatsTaskSender`.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the NATS task sender.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a new `NatsTaskSender` or an error.
    pub async fn new(config: &NatsConfig) -> Result<Self> {
        let client = async_nats::connect(&config.url).await?;
        let ctx = jetstream::new(client);
        Ok(NatsTaskSender { ctx, subject: config.subject.clone() })
    }
}


#[async_trait]
impl TaskSenderBytes for NatsTaskSender {
    /// Sends a task to NATS.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to send as a byte vector.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the task was sent successfully.
    async fn send_task(&self, task: Vec<u8>) -> Result<()> {
        self.ctx.publish(self.subject.clone(), Bytes::from(task)).await?.await?;
        Ok(())
    }
}
