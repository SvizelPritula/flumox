use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router, Server};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

mod api;
mod db;
mod error;
mod extract;

async fn serve(db: Pool) -> Result<()> {
    let api = Router::new()
        .route("/debug", get(api::debug))
        .with_state(db);

    let app = Router::new().nest("/api", api);

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));

    Server::bind(&address)
        .serve(app.into_make_service())
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
    let db = connect_db()?;

    serve(db).await
}
