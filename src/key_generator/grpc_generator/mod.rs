//! This module contains the gRPC implementation of the `KeyGenerationService` trait.
use async_trait::async_trait;
use rust_proto_pkg::generated::key_generator_service_client::KeyGeneratorServiceClient;
use tonic::Code;
use tonic::transport::Channel;
use tonic_tracing_opentelemetry::middleware::client::OtelGrpcLayer;
use tower::ServiceBuilder;
use crate::config::GRPCKeyGeneratorConfig;
use crate::key_generator::error::GeneratorError;
use crate::key_generator::KeyGenerationService;


type KeyGenClient = KeyGeneratorServiceClient<tonic_tracing_opentelemetry::middleware::client::OtelGrpcService<Channel>>;

/// This struct is a gRPC client for the key generator service.
#[derive(Clone, Debug)]
pub struct GRPCGenerator {
    /// We have the client created once and reused it for each request.
    /// This is efficient because the client internally manages a connection pool.
    /// Cloning the client is a cheap operation that just creates a new handle to the same
    /// underlying connection pool.
    client: KeyGenClient,
}


impl GRPCGenerator {
    /// Creates a new `GRPCGenerator`.
    ///
    /// # Arguments
    ///
    /// * `conf` - The configuration for the gRPC generator.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a new `GRPCGenerator` or a `GeneratorError`.
    pub async fn new(conf: &GRPCKeyGeneratorConfig) -> Result<Self, GeneratorError> {
        // 1. Establish the connection once.
        let channel = Channel::from_shared(conf.url.clone())
            .map_err(|err| GeneratorError::UnknownError(err.to_string()))?
            .connect()
            .await
            .map_err(|_| GeneratorError::ConnectionError)?;

        // 2. Apply middleware layers to the channel.
        let layered_channel = ServiceBuilder::new()
            .layer(OtelGrpcLayer)
            .service(channel);

        // 3. Create the client with the layered channel.
        let client = rust_proto_pkg::generated::key_generator_service_client::KeyGeneratorServiceClient::new(layered_channel);

        // 4. Return a new instance of our struct containing the client.
        Ok(GRPCGenerator { client })
    }
}


#[async_trait]
impl KeyGenerationService for GRPCGenerator {
    /// Asynchronously generates a new key.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `String` representing the generated key,
    /// or a `GeneratorError` if key generation fails.
    async fn generate_key(&self) -> Result<String, GeneratorError> {
        // Clone the client. This is a cheap operation that just
        // creates a new handle to the same underlying connection pool.
        let mut client = self.client.clone();

        let res = client.generate_key(rust_proto_pkg::generated::GenerateKeyRequest {}).await.map_err(
            |err| match err.code() {
                Code::InvalidArgument => GeneratorError::BadRequest,
                Code::PermissionDenied => GeneratorError::NotPermission,
                Code::Unavailable => GeneratorError::ConnectionError,
                _ => GeneratorError::UnknownError(err.to_string()),
            }
        )?;

        Ok(res.into_inner().key)
    }
}




/*
#[derive(Clone, Debug)]
pub struct GRPCGenerator {
    url: String,
}


impl GRPCGenerator {
    /// Creates a new `GRPCGenerator` with the specified URL.
    pub fn new(conf: &GRPCKeyGeneratorConfig) -> Result<Self, Box<dyn Error>> {
        Ok(Self { url: conf.url.clone() })
    }
}



#[async_trait]
impl KeyGenerationService for GRPCGenerator {
    async fn generate_key(&self) -> Result<String, GeneratorError> {
        let channel = Channel::from_shared(self.url.clone())
            .map_err(|err| GeneratorError::UnknownError(err.to_string()))?
            .connect()
            .await.map_err(|_| GeneratorError::ConnectionError)?; //Devskim: ignore DS137138

        let channel = ServiceBuilder::new().layer(OtelGrpcLayer).service(channel);

        let mut client = rust_proto_pkg::generated::key_generator_service_client::KeyGeneratorServiceClient::new(channel);

        let res = client.generate_key(rust_proto_pkg::generated::GenerateKeyRequest {}).await.map_err(
            |err| match err.code() {
                Code::InvalidArgument => GeneratorError::BadRequest,
                Code::PermissionDenied => GeneratorError::NotPermission,
                Code::Unavailable => GeneratorError::ConnectionError,
                _ => GeneratorError::UnknownError(err.to_string()),
            }
        )?;
        Ok(res.into_inner().key)
    }
}


 */