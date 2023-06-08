use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use flumox::StateMismatchError;
use serde::Serialize;
use thiserror::Error;
use time_expr::EvalError;

use std::error::Error;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("failed to comunicate with database")]
    Database {
        #[from]
        source: tokio_postgres::Error,
    },
    #[error("failed to get client from pool")]
    Pool,
    #[error("instance and state type does not match")]
    BadStateType {
        #[from]
        source: StateMismatchError,
    },
    #[error("failed to evaluate expression")]
    Eval {
        #[from]
        source: EvalError,
    },
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        eprintln!("Error: {self}");

        let mut error: &dyn Error = &self;

        while let Some(source) = error.source() {
            println!("Caused by: {source}");
            error = source
        }

        #[derive(Serialize)]
        #[serde(rename_all = "kebab-case")]
        enum Type {
            Database,
            BadState,
        }

        #[derive(Serialize)]
        struct Payload {
            reason: Type,
        }

        let response = Payload {
            reason: match self {
                InternalError::Database { .. } => Type::Database,
                InternalError::Pool => Type::Database,
                InternalError::BadStateType { .. } => Type::BadState,
                InternalError::Eval { .. } => Type::BadState,
            },
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
    }
}
