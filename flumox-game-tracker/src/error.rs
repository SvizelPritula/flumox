use anyhow::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::html;
use tracing::error;

use crate::parts::page;

pub struct InternalError(Error);

impl<T: Into<Error>> From<T> for InternalError {
    fn from(value: T) -> Self {
        InternalError(value.into())
    }
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        let error = self.0;

        error!("{error}");

        let content = page(
            "Internal error",
            html!(
                h1 { "Internal error" }
                pre { (error) }
            ),
        );

        (StatusCode::INTERNAL_SERVER_ERROR, content).into_response()
    }
}
