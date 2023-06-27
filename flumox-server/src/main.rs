use std::{iter, net::SocketAddr, path::PathBuf};

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router, Server,
};
use clap::Parser;
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
    compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer, services::ServeDir,
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

async fn serve(db: Pool, port: u16, serve: Option<PathBuf>) -> Result<()> {
    let api = Router::new()
        .route("/login", post(api::login))
        .route("/me", get(api::me))
        .route("/view", get(api::view))
        .route("/action", post(api::submit))
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

fn connect_db(config: Config) -> Result<Pool> {
    let manager = Manager::new(config, NoTls);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}

#[derive(Debug, Parser)]
/// A server for hosting puzzle hunts
struct Options {
    /// The port to listen on
    #[arg(long, default_value_t = 8000, env)]
    port: u16,
    /// A connection string to a Postgres database
    #[arg(
        long,
        default_value = "host='/run/postgresql' user=dev dbname=flumox",
        env = "PG_CONFIG"
    )]
    db: Config,
    /// A directory to serve at server root
    #[arg(long)]
    serve: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::parse();

    let stdout = fmt::layer().compact().with_filter(LevelFilter::INFO);
    registry().with(stdout).init();

    let db = connect_db(options.db)?;

    serve(db, options.port, options.serve).await
}
