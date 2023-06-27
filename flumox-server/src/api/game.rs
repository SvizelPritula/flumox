use std::time::Duration;

use axum::Json;
use deadpool_postgres::Client;
use flumox::{
    Action, ActionContext, ActionEffect, ActionError, Cache, Environment, StateMismatchError, Toast,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;
use tokio::time::sleep;
use tokio_postgres::{error::SqlState, IsolationLevel};
use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    db::{load_state, set_state, LoadStateError},
    error::InternalError,
    extract::DbConnection,
    session::Session,
    types::WidgetInstance,
    view::render,
};

pub async fn view(
    Session { game, team }: Session,
    DbConnection(db): DbConnection,
) -> Result<Json<Vec<WidgetInstance>>, InternalError> {
    async fn run(
        mut db: Client,
        game: Uuid,
        team: Uuid,
        time: OffsetDateTime,
    ) -> Result<Vec<WidgetInstance>, InternalError> {
        let mut db = db.transaction().await?;
        let (game, meta) = load_state(&mut db, game, team).await?;
        db.commit().await?;

        Ok(render(&game, &meta, time)?)
    }

    let time = OffsetDateTime::now_utc();

    match run(db, game, team, time).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            error!("Failed to render view: {err}");
            Err(err)
        }
    }
}

const MAX_RETRIES: u32 = 8;
// Doubles after every transaction
// If all retries fail, it will sleep for a one second total
const INITIAL_RETRY_DELAY_MILLIS: u64 = 2;

#[derive(Debug, Clone, Deserialize)]
pub struct Submission {
    widget: Uuid,
    #[serde(flatten)]
    action: Action,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "result")]
pub enum SubmissionResponse {
    Success { toast: Option<Toast> },
    NotPossible,
    DispatchFailed,
}

pub async fn submit(
    Session { game, team }: Session,
    DbConnection(mut db): DbConnection,
    Json(Submission { widget, action }): Json<Submission>,
) -> Result<Json<SubmissionResponse>, InternalError> {
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
            .ok_or(ActionError::UnknownWidget)?;

        let instance = game_state
            .instances
            .get(ident)
            .ok_or(ActionError::UnknownWidget)?;

        let mut cache = Cache::default();
        let env = Environment::new(&game_state, &mut cache, ident);
        let ctx = ActionContext { env, time };

        let ActionEffect { new_state, toast } = instance.submit(action, ctx)?;

        if let Some(state) = new_state {
            set_state(&mut db, game, team, widget, state).await?;
        }

        db.commit().await?;

        Ok(toast)
    }

    let time = OffsetDateTime::now_utc();
    let mut retry_count = 0u32;

    let result = loop {
        match run(&mut db, game, team, widget, &action, time).await {
            Ok(toast) => break Ok(SubmissionResponse::Success { toast }),
            Err(ProcessActionError::Action(ActionError::NotPossible)) => {
                break Ok(SubmissionResponse::NotPossible)
            }
            Err(ProcessActionError::Action(
                ActionError::UnknownWidget | ActionError::WidgetMismatch,
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
                    if retry_count < MAX_RETRIES {
                        warn!("retrying transaction due to error: {error}");

                        sleep(Duration::from_millis(INITIAL_RETRY_DELAY_MILLIS << retry_count)).await;

                        retry_count += 1;
                    } else {
                        break Err(error.into());
                    }
                }
                _ => break Err(error.into()),
            },
        }
    };

    match result {
        Ok(r) => Ok(Json(r)),
        Err(error) => {
            error!("failed to evaluate action: {error}");
            Err(error)
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
