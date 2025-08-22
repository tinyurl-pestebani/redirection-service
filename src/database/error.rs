//! This module defines the errors that can occur in the database layer.
use axum::http::StatusCode;

use thiserror::Error;

/// This enum represents the different errors that can occur in the database layer.
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// An error indicating that a key was not found in the database.
    #[error("Key not found: {0}")]
    NotExist (String),
    /// An error indicating that a feature is not implemented.
    #[error("Unimplemented error")]
    Unimplemented,
    /// An error indicating that a connection with the database failed.
    #[error("Connection with Database failed: {0}")]
    UnavailableError(String),
    /// An error indicating that an unknown error occurred.
    #[error("Unknown error: {0}")]
    UnknownError(String),
}


/// Implements the conversion from `DatabaseError` to (StatusCode, String).
/// This allows `DatabaseError` to be used as a return type handler services.
impl From<DatabaseError> for (StatusCode, String) {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotExist(key_id) => (StatusCode::NOT_FOUND, key_id),
            DatabaseError::Unimplemented => (StatusCode::NOT_IMPLEMENTED, err.to_string()),
            DatabaseError::UnavailableError(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            DatabaseError::UnknownError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_error_conversion() {
        let not_exist_error = DatabaseError::NotExist("123456ab".to_string());
        let status: (StatusCode, String) = not_exist_error.into();
        assert_eq!(status.0, StatusCode::NOT_FOUND);
        assert_eq!(status.1, "123456ab");

        let not_imp_error = DatabaseError::Unimplemented;
        let status: (StatusCode, String) = not_imp_error.into();
        assert_eq!(status.0, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(status.1, "Unimplemented error");

        let not_av_error = DatabaseError::UnavailableError("service not available".to_string());
        let status: (StatusCode, String) = not_av_error.into();
        assert_eq!(status.0, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(status.1, "service not available");

        let internal_error = DatabaseError::UnknownError("internal error".to_string());
        let status: (StatusCode, String) = internal_error.into();
        assert_eq!(status.0, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status.1, "internal error");
    }
}
