use std::{iter, net::SocketAddr, path::PathBuf};

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router, Server,
};
use http::{
    header::{
        CACHE_CONTROL, CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS,
        X_FRAME_OPTIONS,
    },
    HeaderValue,
};
use tower_http::{
    compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer, services::ServeDir,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tracing::info;

use crate::{api, session::X_AUTH_TOKEN, state::State};

pub async fn serve(state: State, port: u16, serve: Option<PathBuf>) -> Result<()> {
    let api = Router::new()
        .route("/login", post(api::login))
        .route("/me", get(api::me))
        .route("/view", get(api::view))
        .route("/action", post(api::submit))
        .fallback(api::not_found)
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'"),
        ))
        .with_state(state);

    let app = if let Some(path) = serve {
        Router::new().fallback_service(ServeDir::new(path)).layer(
            SetResponseHeaderLayer::if_not_present(
                CACHE_CONTROL,
                HeaderValue::from_static("max-age=300"),
            ),
        )
    } else {
        Router::new()
    };

    let app = app
        .nest("/api/", api)
        .layer(CompressionLayer::new().deflate(true).gzip(true).br(true))
        .layer(SetResponseHeaderLayer::if_not_present(
            X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(SetSensitiveHeadersLayer::new(iter::once(
            X_AUTH_TOKEN.clone(),
        )));

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));

    info!(%address, "Server started");

    Server::bind(&address)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
