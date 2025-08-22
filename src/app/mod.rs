//! This module contains the application state and handlers for the redirection service.

pub(crate) mod handlers;

use std::sync::Arc;
use anyhow::Result;
use crate::database::Database;
use crate::key_generator::KeyGenerationService;
use crate::task_sender::TaskSender;

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    db_layer: Arc<dyn Database>,
    task_sender: Arc<dyn TaskSender>,
    key_generator: Arc<dyn KeyGenerationService>,
}


impl AppState {
    pub async fn new(
        db_layer: Arc<dyn Database>,
        task_sender: Arc<dyn TaskSender>,
        key_generator: Arc<dyn KeyGenerationService>,
    ) -> Result<Self> {
        Ok(AppState { db_layer, task_sender, key_generator })
    }
}
