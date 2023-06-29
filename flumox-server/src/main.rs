use std::path::PathBuf;

use anyhow::Result;
use channel_map::ChannelMap;
use clap::Parser;
use deadpool_postgres::{Manager, Pool};
use message::{ChannelSender, Channels};
use server::serve;
use state::State;
use tokio::sync::watch;
use tokio_postgres::{Config, NoTls};
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer,
};

mod action;
mod api;
mod db;
mod error;
mod extract;
mod message;
mod server;
mod session;
mod state;
mod types;
mod view;

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

fn connect_db(config: Config) -> Result<Pool> {
    let manager = Manager::new(config, NoTls);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}

fn start_message_listener(config: Config) -> Channels {
    let (sender, receiver) = watch::channel(false);
    let game = ChannelMap::new(16);
    let team = ChannelMap::new(16);

    tokio::spawn(message::listen(
        config,
        ChannelSender {
            online: sender,
            invalidate_game: game.clone(),
            invalidate_team: team.clone(),
        },
    ));

    Channels {
        online: receiver,
        invalidate_game: game,
        invalidate_team: team,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::parse();

    let stdout = fmt::layer().compact().with_filter(LevelFilter::INFO);
    registry().with(stdout).init();

    let pool = connect_db(options.db.clone())?;
    let channels = start_message_listener(options.db);

    let state = State { pool, channels };

    serve(state, options.port, options.serve).await
}
