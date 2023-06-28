use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use deadpool_postgres::{Manager, Pool};
use server::serve;
use tokio_postgres::{Config, NoTls};
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer,
};

mod api;
mod db;
mod error;
mod extract;
mod server;
mod session;
mod types;
mod view;

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
