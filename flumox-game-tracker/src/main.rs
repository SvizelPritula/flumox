use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    extract::FromRef,
    http::{
        header::{CACHE_CONTROL, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS, STRICT_TRANSPORT_SECURITY},
        HeaderValue,
    },
    routing::get,
    Router, Server,
};
use clap::{ArgAction, Parser};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};
use tower_http::{
    compression::CompressionLayer, set_header::SetResponseHeaderLayer, trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer,
};

use crate::parts::not_found;

mod db;
mod error;
mod parts;
mod routes;

#[derive(Debug, Clone, FromRef)]
struct State {
    db: Pool,
}

async fn serve(state: State, port: u16, creds: Option<Credentials>) -> Result<()> {
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/:game/", get(routes::game))
        .route("/:game/:team/", get(routes::team))
        .fallback(|| async { not_found("Page") });

    let app = if let Some(creds) = creds {
        app.layer(ValidateRequestHeaderLayer::basic(&creds.user, &creds.pass))
    } else {
        app
    };

    let app = app
        .layer(CompressionLayer::new().deflate(true).gzip(true).br(true))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
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
        .layer(SetResponseHeaderLayer::if_not_present(
            STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=36288000"),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));

    info!(%address, "Server started");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Credentials {
    user: String,
    pass: String,
}

#[derive(Debug, Parser)]
/// A server for tracking teams progress in Flumox
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
    /// Protect server with HTTP Basic auth with this password
    #[arg(long, env = "AUTH_PASS")]
    pass: Option<String>,
    /// Username for HTTP Basic auth, only used if password set
    #[arg(long, default_value = "flumox", env = "AUTH_USER")]
    user: String,
    /// Whether to use ANSI codes in output
    #[arg(long, default_value_t = true, env = "LOG_COLOR", action=ArgAction::Set)]
    color: bool,
}

fn connect_db(config: Config) -> Result<Pool> {
    let manager = Manager::new(config, NoTls);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::parse();

    let stdout = fmt::layer()
        .with_ansi(options.color)
        .with_filter(LevelFilter::INFO);

    registry().with(stdout).init();

    let db = connect_db(options.db.clone())?;
    let state = State { db };

    let creds = options.pass.map(|pass| Credentials {
        pass,
        user: options.user,
    });

    serve(state, options.port, creds).await
}
