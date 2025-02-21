use std::time::Duration;

use deadpool_postgres::Client;
use flumox::{
    Action, ActionContext, ActionEffect, ActionError, Cache, Environment, StateMismatchError, Toast,
};
use serde::Serialize;
use thiserror::Error;
use time::OffsetDateTime;
use tokio::time::sleep;
use tokio_postgres::{error::SqlState, IsolationLevel};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    db::{add_action, load_state, set_state, LoadStateError},
    error::InternalError,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "result")]
pub enum SubmissionResponse {
    Success { toast: Option<Toast> },
    NotPossible,
    DispatchFailed,
}

pub async fn submit_action(
    db: &mut Client,
    game: Uuid,
    team: Uuid,
    widget: Uuid,
    action: Action,
) -> Result<SubmissionResponse, InternalError> {
    const RETRY_DURATIONS: [Duration; 6] = [
        Duration::ZERO,
        Duration::from_millis(16),
        Duration::from_millis(32),
        Duration::from_millis(64),
        Duration::from_millis(128),
        Duration::from_millis(256),
    ];

    async fn run(
        db: &mut Client,
        game: Uuid,
        team: Uuid,
        widget: Uuid,
        action: &Action,
        time: OffsetDateTime,
    ) -> Result<Option<Toast>, ProcessActionError> {
        let mut db = db
            .build_transaction()
            .isolation_level(IsolationLevel::Serializable)
            .start()
            .await?;

        let (game_state, meta) = load_state(&mut db, game, team).await?;

        let (ident, _) = meta
            .iter()
            .find(|(_, meta)| meta.id == widget)
            .ok_or(ActionError::UnknownIdent)?;

        let instance = game_state
            .instances
            .get(ident)
            .ok_or(ActionError::UnknownIdent)?;

        let mut cache = Cache::default();
        let env = Environment::new(&game_state, &mut cache, ident);
        let ctx = ActionContext { env, time };

        let ActionEffect { new_state, toast } = instance.submit(action, ctx)?;

        if let Some(state) = new_state {
            set_state(&mut db, game, team, widget, state).await?;
        }

        add_action(&mut db, game, team, widget, time, action).await?;

        db.commit().await?;

        Ok(toast)
    }

    let time = OffsetDateTime::now_utc();
    let mut retries = RETRY_DURATIONS.iter().copied();

    info!(%game, %team, %widget, %time, "Action by {team} for {widget} received: {action:?}");

    loop {
        match run(db, game, team, widget, &action, time).await {
            Ok(toast) => break Ok(SubmissionResponse::Success { toast }),
            Err(ProcessActionError::Action(ActionError::NotPossible)) => {
                break Ok(SubmissionResponse::NotPossible)
            }
            Err(ProcessActionError::Action(
                ActionError::UnknownIdent | ActionError::WidgetMismatch,
            )) => {
                break Ok(SubmissionResponse::DispatchFailed);
            }
            Err(ProcessActionError::Action(ActionError::Eval(error))) => {
                break Err(error.into());
            }
            Err(ProcessActionError::StateMismatch(error)) => {
                break Err(error.into());
            }
            Err(ProcessActionError::Db(error)) => match error.code() {
                Some(&SqlState::T_R_SERIALIZATION_FAILURE | &SqlState::T_R_DEADLOCK_DETECTED) => {
                    if let Some(delay) = retries.next() {
                        warn!(%game, %team, %widget, %time,
                            "Retrying transaction in {delay} due to error: {error}",
                            delay = delay.as_millis(),
                        );

                        sleep(delay).await;
                    } else {
                        error!(%game, %team, %widget, %time, "Exhausted transaction retries with error: {error}");

                        break Err(error.into());
                    }
                }
                _ => break Err(error.into()),
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum ProcessActionError {
    #[error("failed to process action: {0}")]
    Action(#[from] ActionError),
    #[error(transparent)]
    StateMismatch(#[from] StateMismatchError),
    #[error("failed to comunicate with database: {0}")]
    Db(#[from] tokio_postgres::Error),
}

impl From<LoadStateError> for ProcessActionError {
    fn from(value: LoadStateError) -> Self {
        match value {
            LoadStateError::Database(e) => e.into(),
            LoadStateError::StateMismatch(e) => e.into(),
        }
    }
}
