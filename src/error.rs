//! Provides the `BlueScoutError` enum, which encapsulates various error types

use core::fmt::Display;
use leptos::server_fn::error::{FromServerFnError, ServerFnErrorErr};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// `BlueScoutError` is a custom error enum for the `BlueScout` application.
///
/// It encapsulates different types of errors that can occur within the
/// application, such as server function errors, database errors, API errors,
/// and custom errors.
#[derive(Debug, Clone, Error, Deserialize, Serialize)]
#[non_exhaustive]
pub enum BlueScoutError {
    /// Represents an error that occurs during server function execution.
    #[error("Server Error: {0}")]
    ServerFn(ServerFnErrorErr),
    /// Represents an error that occurs during database operations.
    #[error("Database Error: {0}")]
    DatabaseError(String),
    /// Represents an error that occurs during API calls.
    #[error("API Error: {0}")]
    ApiError(String),
    /// Represents a custom error with a specific message.
    #[error("Error: {0}")]
    Custom(String),
}

impl BlueScoutError {
    /// Creates a `BlueScoutError::ServerFn` from a `ServerFnErrorErr`.
    ///
    /// # Arguments
    ///
    /// * `err` - The `ServerFnErrorErr` to convert.
    #[must_use]
    pub const fn server_fn(err: ServerFnErrorErr) -> Self {
        Self::ServerFn(err)
    }

    /// Creates a `BlueScoutError::DatabaseError` from any error that implements
    /// `Display`.
    ///
    /// # Arguments
    ///
    /// * `err` - The error to convert.
    #[must_use]
    pub fn database_error<E>(err: E) -> Self
    where
        E: Display,
    {
        Self::DatabaseError(err.to_string())
    }

    /// Creates a `BlueScoutError::ApiError` from any error that implements
    /// `Display`.
    ///
    /// # Arguments
    ///
    /// * `err` - The error to convert.
    #[must_use]
    pub fn api_error<E>(err: E) -> Self
    where
        E: Display,
    {
        Self::ApiError(err.to_string())
    }

    /// Creates a `BlueScoutError::Custom` from any error that implements
    /// `Display`.
    ///
    /// # Arguments
    ///
    /// * `err` - The error to convert.
    #[must_use]
    pub fn custom<E>(err: E) -> Self
    where
        E: Display,
    {
        Self::Custom(err.to_string())
    }
}

impl FromServerFnError for BlueScoutError {
    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}

#[cfg(feature = "ssr")]
impl From<anyhow::Error> for BlueScoutError {
    fn from(err: anyhow::Error) -> Self {
        Self::Custom(err.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<duckdb::Error> for BlueScoutError {
    fn from(err: duckdb::Error) -> Self {
        Self::DatabaseError(err.to_string())
    }
}
