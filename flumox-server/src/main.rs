use std::{iter, net::SocketAddr};

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router, Server,
};
use deadpool_postgres::{Manager, Pool};
use http::{
    header::{
        CACHE_CONTROL, CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS,
        X_FRAME_OPTIONS,
    },
    HeaderValue,
};
use tokio_postgres::{Config, NoTls};
use tower_http::{
    compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer,
};

use crate::{api::not_found, session::X_AUTH_TOKEN};

mod api;
mod db;
mod error;
mod extract;
mod session;
mod types;
mod view;

async fn serve(db: Pool) -> Result<()> {
    let api = Router::new()
        .route("/login", post(api::login))
        .route("/me", get(api::me))
        .route("/view", get(api::view))
        .fallback(not_found)
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'"),
        ))
        .with_state(db);

    let app = Router::new()
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

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));

    info!(%address, "Server started");

    Server::bind(&address)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

fn connect_db() -> Result<Pool> {
    let mut config = Config::new();
    config.host_path("/run/postgresql");
    config.user("dev");
    config.dbname("flumox");

    let manager = Manager::new(config, NoTls);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let stdout = fmt::layer().compact().with_filter(LevelFilter::INFO);

    registry().with(stdout).init();

    let db = connect_db()?;

    serve(db).await
}
