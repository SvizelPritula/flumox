use std::collections::HashMap;

use deadpool_postgres::Transaction;
use flumox::{Config, GameState, State, StateMismatchError};
use indexmap::IndexMap;
use thiserror::Error;
use tokio_postgres::{types::Json, Error};
use uuid::Uuid;

use crate::{error::InternalError, types::InstanceMetadata};

pub async fn load_state(
    db: &mut Transaction<'_>,
    game: Uuid,
    team: Uuid,
) -> Result<(GameState, HashMap<String, InstanceMetadata>), LoadStateError> {
    const LOAD_STATE: &str = concat!(
        "SELECT widget.id, widget.ident, widget.config, state.state ",
        "FROM widget LEFT JOIN state ",
        "ON state.game=widget.game AND state.widget=widget.id AND state.team=$2 ",
        "WHERE widget.game=$1 ",
        "ORDER BY widget.id"
    );

    let statement = db.prepare_cached(LOAD_STATE).await?;
    let rows = db.query(&statement, &[&game, &team]).await?;

    let mut instances = IndexMap::new();
    let mut metadata = HashMap::new();

    for row in rows {
        let id: Uuid = row.try_get(0)?;
        let ident: String = row.try_get(1)?;
        let Json(config): Json<Config> = row.try_get(2)?;

        let instance = if let Some(Json(state)) = row.try_get(3)? {
            config.instance(state)?
        } else {
            config.instance_default()
        };

        instances.insert(ident.clone(), instance);
        metadata.insert(ident, InstanceMetadata { id });
    }

    Ok((GameState { instances }, metadata))
}

pub async fn set_state(
    db: &mut Transaction<'_>,
    game: Uuid,
    team: Uuid,
    widget: Uuid,
    state: State,
) -> Result<(), Error> {
    const SET_STATE: &str = concat!(
        "INSERT INTO state (game, team, widget, state) ",
        "VALUES ($1, $2, $3, $4) ",
        "ON CONFLICT (game, team, widget) ",
        "DO UPDATE SET state=excluded.state"
    );

    let statement = db.prepare_cached(SET_STATE).await?;
    db.execute(&statement, &[&game, &team, &widget, &Json(state)])
        .await?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum LoadStateError {
    #[error("failed to comunicate with database: {0}")]
    Database(#[from] Error),
    #[error(transparent)]
    StateMismatch(#[from] StateMismatchError),
}

impl From<LoadStateError> for InternalError {
    fn from(value: LoadStateError) -> Self {
        match value {
            LoadStateError::Database(e) => e.into(),
            LoadStateError::StateMismatch(e) => e.into(),
        }
    }
}
