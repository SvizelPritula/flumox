use std::{
    cmp::{max, min},
    time::Duration,
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use deadpool_postgres::{Pool, PoolError};
use futures::future::OptionFuture;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;
use time_expr::EvalError;
use tokio::{select, sync::broadcast::error::RecvError, time::sleep};
use tracing::warn;

use crate::{
    db::{load_state, team_by_session_token, LoadStateError},
    error::InternalError,
    message::{Channels, Invalidate},
    session::{Session, SessionToken},
    types::{TeamId, WidgetInstance},
    view::{render, RenderResult},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum IncomingMessage {
    Auth { token: SessionToken },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum OutgoingMessage<'a> {
    MalformedMessage,
    UnknownToken,
    View { widgets: &'a [WidgetInstance] },
}

fn malformed_message() -> Result<Message, serde_json::Error> {
    let payload = serde_json::to_string(&OutgoingMessage::MalformedMessage)?;
    Ok(Message::Text(payload))
}

fn unknown_token() -> Result<Message, serde_json::Error> {
    let payload = serde_json::to_string(&OutgoingMessage::UnknownToken)?;
    Ok(Message::Text(payload))
}

fn views(widgets: &[WidgetInstance]) -> Result<Message, serde_json::Error> {
    let payload = serde_json::to_string(&OutgoingMessage::View { widgets })?;
    Ok(Message::Text(payload))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Validity {
    Valid,
    Expired,
    StateChanged,
}

async fn wait_until(time: OffsetDateTime) {
    loop {
        let now = OffsetDateTime::now_utc();
        let delta = time - now;

        let Ok(delta) = delta.try_into() else {
            // Delta is negative
            break;
        };

        let delta = min(delta, Duration::from_secs(10));
        let delta = max(delta, Duration::from_millis(10));

        sleep(delta).await;
    }
}

async fn run(mut socket: WebSocket, pool: Pool, channels: Channels) -> Result<(), RunSocketError> {
    let token = loop {
        match socket.recv().await.transpose()? {
            Some(Message::Text(payload)) => match serde_json::from_str(&payload) {
                Ok(IncomingMessage::Auth { token }) => break token,
                Err(_) => {
                    socket.send(malformed_message()?).await?;
                    return Ok(());
                }
            },
            Some(_) => {}
            None => return Ok(()),
        }
    };

    let Session { game, team } = {
        let mut db = pool.get().await?;

        let Some(session) = team_by_session_token(&mut db, token).await? else {
            socket.send(unknown_token()?).await?;
            return Ok(());
        };

        session
    };

    let mut reconnect = channels.reconnect.subscribe();
    let mut invalidate_game = channels.invalidate_game.subscribe(game);
    let mut invalidate_team = channels.invalidate_team.subscribe(TeamId { game, team });

    let (mut state, mut meta) = {
        let mut db = pool.get().await?;
        let mut db = db.transaction().await?;
        load_state(&mut db, game, team).await?
    };

    let RenderResult {
        mut widgets,
        mut valid_until,
    } = render(&state, &meta, OffsetDateTime::now_utc())?;

    socket.send(views(&widgets)?).await?;

    loop {
        let validity = select! {
            result = async {
                select!{
                    r = reconnect.recv() => r,
                    r = invalidate_game.recv() => r,
                    r = invalidate_team.recv() => r,
                }
            } => match result {
                Ok(Invalidate) => Validity::StateChanged,
                Err(RecvError::Lagged(_)) => Validity::Valid,
                Err(RecvError::Closed) => break,
            },
            Some(()) = OptionFuture::from(valid_until.map(wait_until)) => {
                Validity::Expired
            },
        };

        if validity == Validity::StateChanged {
            (state, meta) = {
                let mut db = pool.get().await?;
                let mut db = db.transaction().await?;
                load_state(&mut db, game, team).await?
            };
        }

        if validity != Validity::Valid {
            RenderResult {
                widgets,
                valid_until,
            } = render(&state, &meta, OffsetDateTime::now_utc())?;

            socket.send(views(&widgets)?).await?;
        }
    }

    Ok(())
}

pub async fn sync_socket(
    State(pool): State<Pool>,
    State(channels): State<Channels>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_failed_upgrade(|error| {
        warn!("Websocket upgrade failed: {error}");
    })
    .on_upgrade(|socket| async {
        if let Err(error) = run(socket, pool, channels).await {
            warn!("Websocket connection closed due to error: {error}");
        };
    })
}

#[derive(Debug, Error)]
enum RunSocketError {
    #[error("internal error: {0}")]
    Internal(#[from] InternalError),
    #[error("transport error: {0}")]
    Transport(#[from] axum::Error),
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
}

macro_rules! proxy_internal_error {
    ($error: ty) => {
        impl From<$error> for RunSocketError {
            fn from(value: $error) -> Self {
                RunSocketError::Internal(value.into())
            }
        }
    };
}

proxy_internal_error!(tokio_postgres::Error);
proxy_internal_error!(PoolError);
proxy_internal_error!(LoadStateError);
proxy_internal_error!(EvalError);
