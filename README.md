# redirection-service

Simple axum server that creates new shortened urls, and redirects to that url if its shortened url has been requested.


## Getting Started
This service is responsible for creating shortened urls and redirecting to the original url when the shortened url is requested. Once a shortened url is requested, the service will redirect to the original url and sends an event to a task queue for its processing in a task queue.

## URLs

The service exposes the following endpoints:
- `POST /api/v1/create`: Creates a new shortened url. Expects a JSON body with the following structure:
  ```json
  {
    "url": "https://example.com"
  }
  ```
  Returns the endpoint with the shortened URL
  ```
  http://localhost:8081/abc12345
  ```
- `GET /:shortened_url`: Redirects to the original url if the shortened url exists. If it does not exist, returns a 404 error.


## Environment Variables
The service requires the following environment variables to be set:
- `REDIRECTION_SERVICE_PORT`: The port on which the service will run (default: `8081`).
- `SCYLLA_URI`: The ScyllaDB connection string (default to `localhost:9042`).
- `SCYLLA_KEYSPACE`: The ScyllaDB keyspace to use (default: `examples_ks`).
- `SCYLLA_REPLICATION_FACTOR`: The replication factor for the ScyllaDB keyspace (default: `3`).
- `KEY_GENERATION_SERVICE_URL`: The URL of the key generation service (default: `http://localhost:8080`).
- `KEY_GENERATOR_TYPE`: The type of key generator to use (default: `grpc`).
- `NATS_URL`: The NATS server URL (default: `nats://localhost:4222`).
- `NATS_TASK_SUBJECT`: The NATS subject for task queue (default: `tasks.visit`).
- `TASK_SENDER_TYPE`: The type of task sender to use (default: `nats`).
- `DATABASE_TYPE`: The type of database to use (default: `scylla`).

For OpenTelemetry configuration, please refer to the [OpenTelemetry setup repository](https://github.com/tinyurl-pestebani/rust-otel-setup).
