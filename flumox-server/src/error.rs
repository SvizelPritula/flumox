use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use deadpool_postgres::PoolError;
use flumox::StateMismatchError;
use serde::Serialize;
use thiserror::Error;
use time_expr::EvalError;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("failed to comunicate with database: {source}")]
    Database {
        #[from]
        source: tokio_postgres::Error,
    },
    #[error("failed to get client from pool")]
    Pool,
    #[error("instance and state type does not match: {source}")]
    BadStateType {
        #[from]
        source: StateMismatchError,
    },
    #[error("failed to evaluate expression: {source}")]
    Eval {
        #[from]
        source: EvalError,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InternalErrorType {
    Database,
    Config,
}

#[derive(Serialize)]
pub struct ErrorResponse<E> {
    pub reason: E,
}

impl<E> ErrorResponse<E> {
    pub fn new(error: E) -> Self {
        ErrorResponse { reason: error }
    }
}

impl InternalError {
    pub fn public_type(&self) -> InternalErrorType {
        match self {
            InternalError::Database { .. } => InternalErrorType::Database,
            InternalError::Pool => InternalErrorType::Database,
            InternalError::BadStateType { .. } => InternalErrorType::Config,
            InternalError::Eval { .. } => InternalErrorType::Config,
        }
    }
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.public_type())).into_response()
    }
}

impl From<PoolError> for InternalError {
    fn from(value: PoolError) -> Self {
        match value {
            PoolError::Backend(source) => InternalError::Database { source },
            _ => InternalError::Pool,
        }
    }
}
