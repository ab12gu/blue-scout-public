use leptos::server_fn::error::{FromServerFnError, ServerFnErrorErr};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone, Error, Deserialize, Serialize)]
pub enum BlueScoutError {
    #[error("Server Error: {0}")]
    ServerFn(ServerFnErrorErr),
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("API Error: {0}")]
    ApiError(String),
    #[error("Error: {0}")]
    Custom(String),
}

impl BlueScoutError {
    pub fn server_fn(err: ServerFnErrorErr) -> Self {
        Self::ServerFn(err)
    }

    pub fn database_error<E>(err: E) -> Self
    where
        E: Display,
    {
        Self::DatabaseError(err.to_string())
    }

    pub fn api_error<E>(err: E) -> Self
    where
        E: Display,
    {
        Self::ApiError(err.to_string())
    }

    pub fn custom<E>(err: E) -> Self
    where
        E: Display,
    {
        Self::Custom(err.to_string())
    }
}

impl FromServerFnError for BlueScoutError {
    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        Self::ServerFn(err)
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
