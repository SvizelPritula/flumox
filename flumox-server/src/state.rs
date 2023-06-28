use axum::extract::FromRef;
use deadpool_postgres::Pool;

use crate::message::Channels;

#[derive(Debug, Clone, FromRef)]
pub struct State {
    pub pool: Pool,
    pub channels: Channels,
}
