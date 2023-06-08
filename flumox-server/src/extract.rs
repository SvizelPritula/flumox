use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use deadpool_postgres::{Object, Pool, PoolError};

use crate::error::InternalError;

pub struct DbConnection(pub Object);

#[async_trait]
impl FromRequestParts<Pool> for DbConnection {
    type Rejection = InternalError;

    async fn from_request_parts(_parts: &mut Parts, state: &Pool) -> Result<Self, Self::Rejection> {
        match state.get().await {
            Ok(conn) => Ok(DbConnection(conn)),
            Err(error) => Err(match error {
                PoolError::Backend(source) => InternalError::Database { source },
                _ => InternalError::Pool,
            }),
        }
    }
}
