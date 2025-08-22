//! This module contains the handlers for the application routes.
use axum::body::Bytes;
use axum::extract::{Path, State, Request};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;

use tracing::instrument;

use std::time::SystemTime;

use crate::app::AppState;

use rust_proto_pkg;

use tracing::log::{error, warn};

/// The maximum size of the payload for the create_url endpoint.
const MAX_PAYLOAD_SIZE: usize = 5 * 1024; // 5KB

/// The route for creating a new URL.
pub const ROUTE_CREATE_URL: &str = "/api/v1/create";

/// The route for getting a URL.
pub const ROUTE_GET_URL: &str = "/{url_key}";


/// This handler creates a new shortened URL.
/// It takes a JSON payload with a "url" field and returns a shortened URL.
#[instrument(level = "info", target = "create_url", skip(state))]
pub async fn create_url(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();

    let bytes: Bytes = axum::body::to_bytes(body, MAX_PAYLOAD_SIZE).await.map_err(|err| {
        let msg = format!("Error reading request body: {}", err);
        warn!("{}", msg);
        (StatusCode::BAD_REQUEST, msg)
    })?;

    let payload: CreateURLRequest = serde_json::from_slice(&bytes).map_err(|err| {
        let msg = format!("Error deserializing request body: {}", err);
        warn!("{}", msg);
        (StatusCode::BAD_REQUEST, msg)
    })?;

    let key = state.key_generator.generate_key().await?;

    let headers = &parts.headers;
    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost");

    let schema = if let Some(ch) = parts.uri.scheme() {
        ch.to_string()
    } else {
        "http".to_string()
    };

    state.db_layer.insert_key(key.clone(), payload.url).await?;

    let url = format!("{schema}://{host}/{key}");

    Ok((StatusCode::CREATED, url))
}


/// This handler retrieves a URL from a shortened key and redirects the user to it.
/// It also sends a task to a task sender to record the URL visit.
#[instrument(level = "info", target = "get_url", skip(state))]
pub async fn get_url(
    State(state): State<AppState>,
    Path(url_key): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url = state.db_layer.get_key_url(&url_key).await?;
    
    let now_dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    
    state.task_sender.send_task(
        rust_proto_pkg::generated::Task {
            task: Some(
                rust_proto_pkg::generated::task::Task::T1(rust_proto_pkg::generated::InsertRecord {
                    tag: url_key,
                    time: Some(
                        prost_types::Timestamp {
                            seconds: now_dur.as_secs() as i64,
                            nanos: now_dur.subsec_nanos() as i32,
                        }
                    ),
                })
            )
        }
    ).await.unwrap_or_else(|err| {
        error!("Error sending task: {}", err);
    });

    Ok(Redirect::permanent(url.as_str()))
}


#[derive(Deserialize)]
struct CreateURLRequest {
    url: String,
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use anyhow::anyhow;
    use super::*;
    use axum::http::Request;
    use axum::response::{IntoResponse, Response};
    use axum::body::Body;
    use crate::app::AppState;
    use crate::database::MockDatabase;
    use crate::key_generator::MockKeyGenerationService;
    use crate::task_sender::MockTaskSender;

    #[tokio::test]
    async fn test_create_url() {
        // Mock AppState and its dependencies
        let mut db_layer = MockDatabase::new();
        let mut key_generator = MockKeyGenerationService::new();
        let task_sender = MockTaskSender::new();

        db_layer.expect_insert_key().returning(|_, _| Ok(()));
        key_generator.expect_generate_key().returning(|| Ok("12345678".to_string()));

        let state = AppState::new (
            Arc::new(db_layer),
            Arc::new(task_sender),
            Arc::new(key_generator),
        ).await.unwrap();

        // Create a mock request
        let req = Request::builder()
            .method("POST")
            .uri("http://some-host/api/v1/create")
            .body(Body::from(r#"{"url": "http://example.com"}"#))
            .unwrap();

        // Call the handler
        let response = create_url(State(state), req).await;

        // Assert the response
        assert!(response.is_ok());
        let resp: Response = response.unwrap().into_response();
        assert_eq!(resp.status(), StatusCode::CREATED);

        let body_bytes = axum::body::to_bytes(resp.into_body(), 50_usize).await.unwrap();
        assert_eq!(body_bytes, "http://some-host/12345678"); // Assuming the key is generated as "12345678");
    }

    #[tokio::test]
    async fn test_create_url_bad_req() {
        let db_layer = MockDatabase::new();
        let key_generator = MockKeyGenerationService::new();
        let task_sender = MockTaskSender::new();

        let state = AppState::new (
            Arc::new(db_layer),
            Arc::new(task_sender),
            Arc::new(key_generator),
        ).await.unwrap();

        let req = Request::builder()
            .method("POST")
            .uri("http://some-host/api/v1/create")
            // Malformed JSON body
            .body(Body::from(r#"{"url": "http://example.com""#))
            .unwrap();

        let response = create_url(State(state), req).await.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_url() {
        // Mock AppState and its dependencies
        let mut db_layer = MockDatabase::new();
        let mut task_sender = MockTaskSender::new();

        db_layer.expect_get_key_url().returning(|_| Ok("http://example.com".to_string()));
        task_sender.expect_send_task().returning(|_| Ok(()));

        let state = AppState::new (
            Arc::new(db_layer),
            Arc::new(task_sender),
            Arc::new(MockKeyGenerationService::new()),
        ).await.unwrap();

        // Call the handler
        let response = get_url(State(state), Path("12345678".to_string())).await;

        // Assert the response
        assert!(response.is_ok());
        let resp: Response = response.unwrap().into_response();
        assert_eq!(resp.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(resp.headers()["Location"], "http://example.com");
    }

    #[tokio::test]
    async fn test_get_url_err_task() {
        // Mock AppState and its dependencies
        let mut db_layer = MockDatabase::new();
        let mut task_sender = MockTaskSender::new();

        db_layer.expect_get_key_url().returning(|_| Ok("http://example.com".to_string()));
        task_sender.expect_send_task().returning(|_| Err(anyhow!("Error while sending task")));

        let state = AppState::new (
            Arc::new(db_layer),
            Arc::new(task_sender),
            Arc::new(MockKeyGenerationService::new()),
        ).await.unwrap();

        // Call the handler
        let response = get_url(State(state), Path("12345678".to_string())).await;

        // Assert the response
        assert!(response.is_ok());
        let resp: Response = response.unwrap().into_response();
        assert_eq!(resp.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(resp.headers()["Location"], "http://example.com");
    }
}
