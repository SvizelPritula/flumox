use async_trait::async_trait;
use axum::{
    extract::{rejection::TypedHeaderRejectionReason, ConnectInfo, FromRef, FromRequestParts},
    http::{request::Parts, HeaderName, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt, TypedHeader,
};
use deadpool_postgres::{Object, Pool};
use headers::{Header, HeaderValue};
use serde::Serialize;
use thiserror::Error;
use tracing::{error, info, warn};

use crate::{
    db::team_by_session_token,
    error::{ErrorResponse, InternalError},
    session::{Session, SessionToken, X_AUTH_TOKEN},
};

use std::{iter, net::SocketAddr};

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
            Err(err) => {
                let err = err.into();
                error!("Failed to get client from pool: {err}");
                Err(err)
            }
        }
    }
}

struct XAuthToken(SessionToken);

impl Header for XAuthToken {
    fn name() -> &'static HeaderName {
        &X_AUTH_TOKEN
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        let value = value.to_str().map_err(|_| headers::Error::invalid())?;
        let token = value.parse().map_err(|_| headers::Error::invalid())?;
        Ok(XAuthToken(token))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from_str(&self.0.to_string()).expect("base64 is always valid");

        values.extend(iter::once(value));
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    Pool: FromRef<S>,
    S: Sync,
{
    type Rejection = SessionExtractError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        async fn get_session_from_pool(
            pool: &Pool,
            token: SessionToken,
        ) -> Result<Option<Session>, InternalError> {
            let mut db = pool.get().await?;
            team_by_session_token(&mut db, token).await
        }

        match parts.extract().await {
            Err(error) => match error.reason() {
                TypedHeaderRejectionReason::Missing => {
                    Err(SessionExtractError::SessionError(SessionError::NoToken))
                }
                TypedHeaderRejectionReason::Error(_) | _ => Err(SessionExtractError::SessionError(
                    SessionError::MalformedToken,
                )),
            },
            Ok(TypedHeader(XAuthToken(token))) => {
                let pool = Pool::from_ref(state);

                match get_session_from_pool(&pool, token).await {
                    Ok(Some(session)) => Ok(session),
                    Ok(None) => {
                        if let Ok(ConnectInfo(addr)) = parts.extract().await {
                            let addr: SocketAddr = addr;
                            info!(%addr, "Invalid session token supplied");
                        } else {
                            warn!("Failed to get client's IP");
                            info!("Invalid session token supplied");
                        }

                        Err(SessionExtractError::SessionError(
                            SessionError::InvalidToken,
                        ))
                    }
                    Err(err) => {
                        error!("Failed to check session: {err}");
                        Err(SessionExtractError::InternalError(err))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Error, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionError {
    #[error("no session token provided")]
    NoToken,
    #[error("token format invalid")]
    MalformedToken,
    #[error("no session token provided")]
    InvalidToken,
}

#[derive(Debug, Error)]
pub enum SessionExtractError {
    #[error(transparent)]
    InternalError(InternalError),
    #[error(transparent)]
    SessionError(SessionError),
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, Json(ErrorResponse::new(self))).into_response()
    }
}

impl IntoResponse for SessionExtractError {
    fn into_response(self) -> Response {
        match self {
            SessionExtractError::InternalError(e) => e.into_response(),
            SessionExtractError::SessionError(e) => e.into_response(),
        }
    }
}
