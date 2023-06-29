use std::{future::poll_fn, time::Duration};

use tokio::{time::sleep, try_join};
use tokio_postgres::{AsyncMessage, Config, NoTls, Notification};
use tracing::{info, warn};

use crate::types::TeamId;

use super::{ChannelSender, Invalidate, InvalidateMessage};

fn process_message(message: Notification, channels: &ChannelSender) {
    match serde_json::from_str(message.payload()) {
        Ok(InvalidateMessage::Game { game }) => channels.invalidate_game.send(&game, Invalidate),
        Ok(InvalidateMessage::Team { game, team }) => channels
            .invalidate_team
            .send(&TeamId { game, team }, Invalidate),
        Err(error) => {
            warn!(
                payload = message.payload(),
                "Received unknown invalidate message: {error}"
            )
        }
    }
}

async fn run_connection(
    config: &Config,
    channels: &ChannelSender,
) -> Result<(), tokio_postgres::Error> {
    let (client, mut connection) = config.connect(NoTls).await?;

    let subscribe = async {
        let result = client.execute("LISTEN invalidate", &[]).await;

        if result.is_ok() {
            info!("Listening for messages");
            let _ = channels.reconnect.send(Invalidate);
            channels.online.send_replace(true);
        }

        result.map(|_| ())
    };

    let listen = async {
        loop {
            match poll_fn(|c| connection.poll_message(c)).await {
                Some(Ok(AsyncMessage::Notification(message))) => {
                    process_message(message, channels);
                }
                Some(Ok(_)) => {}
                Some(Err(error)) => break Err(error),
                None => break Ok(()),
            }
        }
    };

    try_join!(subscribe, listen).map(|_| ())
}

const RECONNECT_DELAY: Duration = Duration::from_secs(1);

pub async fn listen(config: Config, channels: ChannelSender) {
    loop {
        match run_connection(&config, &channels).await {
            Ok(()) => {
                warn!("Connection to database closed, reconnecting");
            }
            Err(error) => {
                warn!("Listening for messages failed (reconnecting): {error}");
            }
        }

        channels.online.send_replace(false);

        sleep(RECONNECT_DELAY).await
    }
}
