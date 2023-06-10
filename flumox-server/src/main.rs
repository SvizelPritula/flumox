use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router, Server,
};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer,
};

mod api;
mod db;
mod error;
mod extract;
mod session;
mod types;

async fn serve(db: Pool) -> Result<()> {
    let api = Router::new()
        .route("/login", post(api::login))
        .route("/me", get(api::me))
        .route("/view", get(api::view))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let app = Router::new().nest("/api", api);

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
