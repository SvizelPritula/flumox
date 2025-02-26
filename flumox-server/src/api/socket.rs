use std::{
    cmp::{max, min},
    io,
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
use flate2::{write::DeflateEncoder, Compression};
use futures::future::OptionFuture;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;
use time_expr::EvalError;
use tokio::{select, sync::broadcast::error::RecvError, time::sleep};
use tracing::warn;

use crate::{
    db::{load_state, team_by_session_token, LoadStateError},
    error::{InternalError, InternalErrorType},
    message::{Channels, Invalidate},
    session::{Session, SessionToken},
    types::TeamId,
    view::{delta, render, RenderResult, WidgetInstanceDelta},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum IncomingMessage {
    Auth { token: SessionToken, compress: bool },
    Ping,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum OutgoingMessage<'a> {
    MalformedMessage,
    UnknownToken,
    View { widgets: &'a [WidgetInstanceDelta] },
    Pong,
    Error { reason: InternalErrorType },
}

fn text_message(message: &OutgoingMessage) -> Result<Message, RunSocketError> {
    Ok(Message::Text(serde_json::to_string(message)?))
}

fn compressed_message(message: &OutgoingMessage) -> Result<Message, RunSocketError> {
    let mut writer = DeflateEncoder::new(Vec::new(), Compression::best());

    serde_json::to_writer(&mut writer, message)?;

    Ok(Message::Binary(writer.finish()?))
}

fn malformed_message() -> Result<Message, RunSocketError> {
    text_message(&OutgoingMessage::MalformedMessage)
}

fn unknown_token() -> Result<Message, RunSocketError> {
    text_message(&OutgoingMessage::UnknownToken)
}

fn views(widgets: &[WidgetInstanceDelta], compress: bool) -> Result<Message, RunSocketError> {
    let payload = OutgoingMessage::View { widgets };

    if compress {
        compressed_message(&payload)
    } else {
        text_message(&payload)
    }
}

fn pong() -> Result<Message, RunSocketError> {
    text_message(&OutgoingMessage::Pong)
}

fn internal_error(error: &InternalError) -> Result<Message, RunSocketError> {
    text_message(&OutgoingMessage::Error {
        reason: error.public_type(),
    })
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

async fn run(socket: &mut WebSocket, pool: Pool, channels: Channels) -> Result<(), RunSocketError> {
    let (token, compress) = loop {
        match socket.recv().await.transpose()? {
            Some(Message::Text(payload)) => match serde_json::from_str(&payload) {
                Ok(IncomingMessage::Auth { token, compress }) => break (token, compress),
                Ok(IncomingMessage::Ping) => socket.send(pong()?).await?,
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

    socket.send(views(&delta(&widgets, &[]), compress)?).await?;

    loop {
        let validity = select! {
            result = socket.recv() => {
                match result.transpose()? {
                    Some(Message::Text(payload)) => match serde_json::from_str(&payload) {
                        Ok(IncomingMessage::Auth { .. }) => {},
                        Ok(IncomingMessage::Ping) => socket.send(pong()?).await?,
                        Err(_) => {
                            socket.send(malformed_message()?).await?;
                            return Ok(());
                        }
                    },
                    Some(_) => {}
                    None => break,
                }

                Validity::Valid
            }
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
            let RenderResult {
                widgets: new_widgets,
                valid_until: new_valid_until,
            } = render(&state, &meta, OffsetDateTime::now_utc())?;

            socket
                .send(views(&delta(&new_widgets, &widgets), compress)?)
                .await?;

            (widgets, valid_until) = (new_widgets, new_valid_until);
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
    .on_upgrade(|mut socket| async move {
        if let Err(error) = run(&mut socket, pool, channels).await {
            match &error {
                RunSocketError::Internal(error) => {
                    if let Ok(payload) = internal_error(error) {
                        let _ = socket.send(payload).await;
                    }
                }
                RunSocketError::Serialize(_)
                | RunSocketError::Io(_)
                | RunSocketError::Transport(_) => {}
            }

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
    #[error("io error: {0}")]
    Io(#[from] io::Error),
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
