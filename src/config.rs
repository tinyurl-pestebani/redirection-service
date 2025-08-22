//! This module contains the configuration for the redirection service.
use std::env;
use anyhow::{anyhow, Result};

/// This struct contains the configuration for the redirection service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RedirectionServiceConfig {
    /// The port on which the service will listen.
    pub port: u16,
    /// The database configuration.
    pub db_config: DBConfig,
    /// The task sender configuration.
    pub task_sender: TaskSender,
    /// The key generator configuration.
    pub key_generator: KeyGeneratorConfig,
}


/// This struct contains the configuration for a ScyllaDB database.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScyllaDBConfig {
    /// The URL of the ScyllaDB instance.
    pub url : String,
    /// The keyspace to use in ScyllaDB.
    pub keyspace: String,
    /// The replication factor for the keyspace.
    pub replication_factor: i32,
}


/// This enum represents the different database configurations that can be used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DBConfig {
    /// A ScyllaDB configuration.
    ScyllaDB(ScyllaDBConfig),
}


/// This enum represents the different task senders that can be used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskSender {
    /// A NATS configuration.
    Nats(NatsConfig),
}


/// This struct contains the configuration for a NATS task sender.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NatsConfig {
    /// The URL of the NATS server.
    pub url: String,
    /// The subject to which tasks will be sent.
    pub subject: String,
}


/// This enum represents the different key generator configurations that can be used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyGeneratorConfig {
    /// A gRPC key generator configuration.
    GRPCKeyGeneratorConfig(GRPCKeyGeneratorConfig)
}


/// This struct contains the configuration for a gRPC key generator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GRPCKeyGeneratorConfig {
    /// The URL of the gRPC key generator service.
    pub url: String,
}


impl DBConfig {
    /// This function creates a new `DBConfig` from environment variables.
    pub fn from_env() -> Result<Self> {
        let db_type = env::var("DATABASE_TYPE").unwrap_or("scylla".into());
        match db_type.as_str() {
            "scylla" => Ok(DBConfig::ScyllaDB(ScyllaDBConfig::from_env()?)),
            _ => Err(anyhow!("Unsupported database type: {}", db_type)),
        }
    }
}

impl TaskSender {
    /// This function creates a new `TaskSender` from environment variables.
    pub fn from_env() -> Result<Self> {
        let task_sender_type = env::var("TASK_SENDER_TYPE").unwrap_or("nats".into());
        match task_sender_type.as_str() {
            "nats" => Ok(TaskSender::Nats(NatsConfig::from_env()?)),
            _ => Err(anyhow!("Unsupported task sender type: {}", task_sender_type)),
        }
    }
}

impl NatsConfig {
    /// This function creates a new `NatsConfig` from environment variables.
    pub fn from_env() -> Result<Self> {
        let url = env::var("NATS_URL").unwrap_or("nats://localhost:4222".into());
        let subject = env::var("NATS_TASK_SUBJECT").unwrap_or("tasks.visit".into());
        Ok(Self { url, subject })
    }
}

impl KeyGeneratorConfig {
    /// This function creates a new `KeyGeneratorConfig` from environment variables.
    pub fn from_env() -> Result<Self> {
        let key_generator_type = env::var("KEY_GENERATOR_TYPE").unwrap_or("grpc".into());
        match key_generator_type.as_str() {
            "grpc" => Ok(KeyGeneratorConfig::GRPCKeyGeneratorConfig(GRPCKeyGeneratorConfig::from_env()?)),
            _ => Err(anyhow!("Unsupported key_generator type: {}", key_generator_type)),
        }
    }
}

impl GRPCKeyGeneratorConfig {
    /// This function creates a new `GRPCKeyGeneratorConfig` from environment variables.
    pub fn from_env() -> Result<Self> {
        let url = env::var("KEY_GENERATION_SERVICE_URL").unwrap_or("http://localhost:8080".into());
        Ok(Self { url })
    }
}


impl ScyllaDBConfig {
    /// This function creates a new `ScyllaDBConfig` from environment variables.
    pub fn from_env() -> Result<Self> {
        let url = env::var("SCYLLA_URI").unwrap_or("localhost:9042".into());
        let keyspace = env::var("SCYLLA_KEYSPACE").unwrap_or("examples_ks".into());
        let replication_factor = env::var("SCYLLA_REPLICATION_FACTOR")
            .unwrap_or("3".into())
            .parse()?;

        Ok(Self {
            url,
            keyspace,
            replication_factor,
        })
    }
}


impl RedirectionServiceConfig {
    /// This function creates a new `RedirectionServiceConfig` from environment variables.
    pub fn from_env() -> Result<Self> {
        let port = env::var("REDIRECTION_SERVICE_PORT")
            .unwrap_or_else(|_| "8081".to_string())
            .parse::<u16>()?;
        
        let db_config: DBConfig = DBConfig::from_env()?;
        let task_sender: TaskSender = TaskSender::from_env()?;
        let key_generator: KeyGeneratorConfig = KeyGeneratorConfig::from_env()?;
        
        Ok(Self {
            port,
            db_config,
            task_sender,
            key_generator,
        })
    }
}
