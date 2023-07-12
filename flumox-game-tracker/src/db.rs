use deadpool_postgres::Transaction;
use flumox::{Config, State, Instance};
use tokio_postgres::types::Json;
use uuid::Uuid;

use crate::error::InternalError;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
}

pub async fn games(db: &mut Transaction<'_>) -> Result<Vec<Game>, InternalError> {
    const GAMES: &str = "SELECT id, name FROM game";

    let stmt = db.prepare_cached(GAMES).await?;
    let games = db.query(&stmt, &[]).await?;

    games
        .into_iter()
        .map(|r| {
            Ok(Game {
                id: r.try_get(0)?,
                name: r.try_get(1)?,
            })
        })
        .collect()
}

pub async fn game_name(
    db: &mut Transaction<'_>,
    game: Uuid,
) -> Result<Option<String>, InternalError> {
    const GAME: &str = "SELECT name FROM game WHERE id = $1";

    let stmt = db.prepare_cached(GAME).await?;
    let game = db.query_opt(&stmt, &[&game]).await?;

    game.map(|r| Ok(r.try_get(0)?)).transpose()
}

#[derive(Debug, Clone)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
}

pub async fn teams(db: &mut Transaction<'_>, game: Uuid) -> Result<Vec<Team>, InternalError> {
    const TEAMS: &str = "SELECT id, name FROM team WHERE game = $1";

    let stmt = db.prepare_cached(TEAMS).await?;
    let games = db.query(&stmt, &[&game]).await?;

    games
        .into_iter()
        .map(|r| {
            Ok(Team {
                id: r.try_get(0)?,
                name: r.try_get(1)?,
            })
        })
        .collect()
}

pub async fn team_name(
    db: &mut Transaction<'_>,
    game: Uuid,
    team: Uuid,
) -> Result<Option<String>, InternalError> {
    const TEAM: &str = "SELECT name FROM team WHERE game = $1 AND id = $2";

    let stmt = db.prepare_cached(TEAM).await?;
    let team = db.query_opt(&stmt, &[&game, &team]).await?;

    team.map(|r| Ok(r.try_get(0)?)).transpose()
}

#[derive(Debug, Clone)]
pub struct Widget {
    pub id: Uuid,
    pub ident: String,
    pub instance: Instance,
}

pub async fn states(
    db: &mut Transaction<'_>,
    game: Uuid,
    team: Uuid,
) -> Result<Vec<Widget>, InternalError> {
    const STATES: &str = concat!(
        "SELECT widget.id, widget.ident, state.state, widget.config ",
        "FROM widget LEFT JOIN state ",
        "ON state.game=widget.game AND state.widget=widget.id AND state.team=$2 ",
        "WHERE widget.game=$1 ",
        "ORDER BY widget.priority DESC"
    );

    let stmt = db.prepare_cached(STATES).await?;
    let team = db.query(&stmt, &[&game, &team]).await?;

    team.into_iter()
        .map(|r| {
            let id = r.try_get(0)?;
            let ident = r.try_get(1)?;
            let state: Option<Json<State>> = r.try_get(2)?;
            let Json(config): Json<Config> = r.try_get(3)?;

            let instance = match state {
                Some(Json(state)) => config.instance(state)?,
                None => config.instance_default(),
            };

            Ok(Widget {
                id,
                ident,
                instance,
            })
        })
        .collect()
}
