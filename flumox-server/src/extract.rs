use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use deadpool_postgres::{Object, Pool, PoolError};

use crate::error::InternalError;

pub struct DbConnection(pub Object);

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where
    Pool: FromRef<S>,
    S: Sync,
{
    type Rejection = InternalError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Pool::from_ref(state).get().await {
            Ok(conn) => Ok(DbConnection(conn)),
            Err(error) => Err(match error {
                PoolError::Backend(source) => InternalError::Database { source },
                _ => InternalError::Pool,
            }),
        }
    }
}
