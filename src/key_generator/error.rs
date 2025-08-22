//! This module defines the errors that can occur in the key generator.
use axum::http::StatusCode;
use thiserror::Error;


/// `GeneratorError` defines the error used in the generator module.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum GeneratorError {
    /// An error occurred while connecting to a dependency.
    #[error("Connection error")]
    ConnectionError,
    /// The requested generator was not found.
    #[error("Generator not found")]
    GeneratorNotFound,
    /// The request has no permissions to generate the key.
    #[error("Not Permissions")]
    NotPermission,
    /// The request has an invalid parameter to generate the key.
    #[error("Bad Request generating key")]
    BadRequest,
    /// An unknown or unexpected error occurred.
    #[error("Generator unknown error: {0}")]
    UnknownError(String),
}


/// Implements the conversion from `GeneratorError` to (StatusCode, String).
/// This allows `GeneratorError` to be used as a return type handler services.
impl From<GeneratorError> for (StatusCode, String) {
    fn from(err: GeneratorError) -> Self {
        match err {
            GeneratorError::ConnectionError => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            GeneratorError::GeneratorNotFound => (StatusCode::NOT_FOUND, err.to_string()),
            GeneratorError::NotPermission => (StatusCode::FORBIDDEN, err.to_string()),
            GeneratorError::BadRequest => (StatusCode::BAD_REQUEST, err.to_string()),
            GeneratorError::UnknownError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_error_conversion() {
        let connection_error = GeneratorError::ConnectionError;
        let status: (StatusCode, String) = connection_error.into();
        assert_eq!(status.0, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(status.1, "Connection error");

        let not_found_error = GeneratorError::GeneratorNotFound;
        let status: (StatusCode, String) = not_found_error.into();
        assert_eq!(status.0, StatusCode::NOT_FOUND);
        assert_eq!(status.1, "Generator not found");

        let not_permission_error = GeneratorError::NotPermission;
        let status: (StatusCode, String) = not_permission_error.into();
        assert_eq!(status.0, StatusCode::FORBIDDEN);
        assert_eq!(status.1, "Not Permissions");

        let bad_request_error = GeneratorError::BadRequest;
        let status: (StatusCode, String) = bad_request_error.into();
        assert_eq!(status.0, StatusCode::BAD_REQUEST);
        assert_eq!(status.1, "Bad Request generating key");

        let unknown_error = GeneratorError::UnknownError("Some error".to_string());
        let status: (StatusCode, String) = unknown_error.into();
        assert_eq!(status.0, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status.1, "Some error");
    }
}
