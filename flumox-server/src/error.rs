use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("failed to comunicate with database")]
    Database {
        #[from]
        source: tokio_postgres::Error,
    },
    #[error("failed to get client from pool")]
    Pool,
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        #[serde(rename_all = "kebab-case")]
        enum Type {
            Database,
        }

        #[derive(Serialize)]
        struct Payload {
            reason: Type,
        }

        let response = Payload {
            reason: match self {
                InternalError::Database { .. } => Type::Database,
                InternalError::Pool => Type::Database,
            },
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
    }
}
