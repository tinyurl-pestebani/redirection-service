//! This module provides the `TaskSender` trait and its implementations.
mod nats;
use anyhow::Result;
pub mod layer;

use std::fmt::Debug;
use async_trait::async_trait;
use prost::Message;
use rust_proto_pkg;

#[cfg(test)]
use mockall::automock;

/// A trait for sending tasks.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait TaskSender: Debug + Send + Sync {
    /// Sends a task.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to send.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the task was sent successfully.
    async fn send_task(&self, task: rust_proto_pkg::generated::Task) -> Result<()>;
}


/// A trait that allows sending tasks as byte vectors.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait TaskSenderBytes: Send + Sync + Debug {
    /// Sends a task as a byte vector.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to send as a byte vector.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the task was sent successfully.
    async fn send_task(&self, task: Vec<u8>) -> Result<()>;
}


/// A default implementation of `TaskSender` that uses `TaskSenderBytes`.
/// This implementation encodes the `Task` into bytes and sends it using the `TaskSender` trait.
#[async_trait]
impl <T: TaskSenderBytes> TaskSender for T {
    async fn send_task(&self, task: rust_proto_pkg::generated::Task) -> Result<()> {
        let bts = task.encode_to_vec();
        self.send_task(bts).await
    }
}
