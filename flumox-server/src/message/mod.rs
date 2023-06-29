use channel_map::ChannelMap;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, watch};
use uuid::Uuid;

use crate::types::TeamId;

mod listen;
mod notify;

pub use listen::listen;
pub use notify::invalidate;

#[derive(Debug, Clone, Copy, Default)]
pub struct Invalidate;

#[derive(Debug, Clone)]
pub struct Channels {
    pub online: watch::Receiver<bool>,
    pub reconnect: broadcast::Sender<Invalidate>,
    pub invalidate_game: ChannelMap<Uuid, Invalidate>,
    pub invalidate_team: ChannelMap<TeamId, Invalidate>,
}

#[derive(Debug)]
pub struct ChannelSender {
    pub online: watch::Sender<bool>,
    pub reconnect: broadcast::Sender<Invalidate>,
    pub invalidate_game: ChannelMap<Uuid, Invalidate>,
    pub invalidate_team: ChannelMap<TeamId, Invalidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum InvalidateMessage {
    Game { game: Uuid },
    Team { game: Uuid, team: Uuid },
}
