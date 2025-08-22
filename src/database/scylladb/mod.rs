//! This module provides a connection to a ScyllaDB database.

use std::sync::Arc;
use async_trait::async_trait;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use futures::StreamExt as _;
use tracing::instrument;
use crate::config::ScyllaDBConfig;
use crate::database::Database;
use crate::database::error::DatabaseError;

/// A struct that represents a connection to a ScyllaDB database.
#[derive(Clone, Debug)]
pub struct ScyllaDB {
    session: Arc<Session>,
    scylla_config: ScyllaDBConfig,
}


macro_rules! scylla_execution_to_database_error {
    ($e:expr) => {
        $e.map_err(|e| match e {
            scylla::errors::ExecutionError::ConnectionPoolError(e) => DatabaseError::UnavailableError(e.to_string()),
            scylla::errors::ExecutionError::LastAttemptError(e) => DatabaseError::UnavailableError(e.to_string()),
            scylla::errors::ExecutionError::RequestTimeout(e) => DatabaseError::UnavailableError(format!("{:?}", e)),
            _ => DatabaseError::UnknownError(e.to_string()),
        })
    };
}



impl ScyllaDB {
    /// Creates a new `ScyllaDB` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the ScyllaDB connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `ScyllaDB` instance or a `DatabaseError`.
    pub async fn new(config: &ScyllaDBConfig) -> Result<Self, DatabaseError> {
        let uri = config.url.clone();
        let keyspace = config.keyspace.clone();
        let rep_factor = config.replication_factor;

        let session: Session = SessionBuilder::new()
            .known_node(uri.as_str())
            .build()
            .await.map_err(|err| DatabaseError::UnknownError(err.to_string()))?;

        // TODO: Check NetworkTopologyStrategy
        let create_query = format!("CREATE KEYSPACE IF NOT EXISTS {keyspace} WITH REPLICATION = {{'class': 'NetworkTopologyStrategy', 'replication_factor': {rep_factor}}}");
        scylla_execution_to_database_error!(
            session.query_unpaged(create_query.as_str(), ()
        ).await)?;


        // Create a table if it doesn't exist. The table must contain two columns, one called url key, that is a string, and another one called url_redirect, that is a string. The table must have a default TTL of 30 days.
        scylla_execution_to_database_error!(
            session.query_unpaged(
                format!(
                    "CREATE TABLE IF NOT EXISTS {keyspace}.url_table ( \
                        url_key text, \
                        url_redirect text, \
                        PRIMARY KEY (url_key)) \
                        WITH default_time_to_live = 2592000"), // 2,592,000 seconds = 30 days
                &[]
        ).await)?;

        Ok(Self {session: Arc::new(session), scylla_config: config.clone()})
    }
}


#[async_trait]
impl Database for ScyllaDB {
    /// Retrieves the URL associated with a given key from the database.
    #[instrument(level = "info", target = "ScyllaDB::get_key_url")]
    async fn get_key_url(&self, key_id: &String) -> Result<String, DatabaseError> {
        let query = format!("SELECT url_redirect FROM {}.url_table WHERE url_key = ?", self.scylla_config.keyspace);
        let mut rs = self.session
            .query_iter(query, (key_id,))
            .await
            .map_err(|err| DatabaseError::UnknownError(err.to_string()))?
            .rows_stream::<(String,)>()
            .map_err(|err| DatabaseError::UnknownError(err.to_string()))?;

        if let Some(row) = rs.next().await {
            let row = row.map_err(|err| DatabaseError::UnknownError(err.to_string()))?;
            Ok(row.0)
        } else { 
            Err(DatabaseError::NotExist (key_id.clone()))
        }
    }

    /// Inserts a new key-URL pair into the database.
    #[instrument(level = "info", target = "ScyllaDB::insert_key")]
    async fn insert_key(&self, key_id: String, url: String) -> Result<(), DatabaseError> {
        let query = format!("INSERT INTO {}.url_table (url_key, url_redirect) VALUES (?, ?);", self.scylla_config.keyspace);
        scylla_execution_to_database_error!(
            self.session
                .query_unpaged(query, (key_id, url))
                .await
            )?;
        Ok(())
    }
}
