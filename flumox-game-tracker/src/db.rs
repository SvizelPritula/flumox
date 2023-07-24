use deadpool_postgres::Transaction;
use flumox::{Action, Config, Instance, State};
use time::OffsetDateTime;
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
    let teams = db.query(&stmt, &[&game]).await?;

    teams
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
    let states = db.query(&stmt, &[&game, &team]).await?;

    states
        .into_iter()
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

#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub widget: String,
    pub time: OffsetDateTime,
    pub payload: Action,
}

pub async fn actions(
    db: &mut Transaction<'_>,
    game: Uuid,
    team: Uuid,
) -> Result<Vec<ActionInfo>, InternalError> {
    const ACTIONS: &str = concat!(
        "SELECT widget.ident, action.time, action.payload ",
        "FROM action JOIN widget ",
        "ON action.game=widget.game AND action.widget=widget.id ",
        "WHERE action.game=$1 AND action.team=$2 ",
        "ORDER BY action.time DESC"
    );

    let stmt = db.prepare_cached(ACTIONS).await?;
    let actions = db.query(&stmt, &[&game, &team]).await?;

    actions
        .into_iter()
        .map(|r| {
            let widget = r.try_get(0)?;
            let time = r.try_get(1)?;
            let Json(payload) = r.try_get(2)?;

            Ok(ActionInfo {
                widget,
                time,
                payload,
            })
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct RecentActionInfo {
    pub widget: String,
    pub team: String,
    pub time: OffsetDateTime,
    pub payload: Action,
}

pub async fn recent_actions(
    db: &mut Transaction<'_>,
    game: Uuid,
) -> Result<Vec<RecentActionInfo>, InternalError> {
    const ACTIONS: &str = concat!(
        "SELECT widget.ident, team.name, action.time, action.payload ",
        "FROM action ",
        "JOIN widget ",
        "ON action.game=widget.game AND action.widget=widget.id ",
        "JOIN team ",
        "ON action.game=team.game AND action.team=team.id ",
        "WHERE action.game=$1 ",
        "ORDER BY action.time DESC ",
        "LIMIT 30"
    );

    let stmt = db.prepare_cached(ACTIONS).await?;
    let actions = db.query(&stmt, &[&game]).await?;

    actions
        .into_iter()
        .map(|r| {
            let widget = r.try_get(0)?;
            let team = r.try_get(1)?;
            let time = r.try_get(2)?;
            let Json(payload) = r.try_get(3)?;

            Ok(RecentActionInfo {
                widget,
                team,
                time,
                payload,
            })
        })
        .collect()
}

pub async fn last_solved(
    db: &mut Transaction<'_>,
    game: Uuid,
    team: Uuid,
) -> Result<Vec<String>, InternalError> {
    const SOLVED: &str = concat!(
        "SELECT widget.ident ",
        "FROM state JOIN widget ",
        "ON state.game=widget.game AND state.widget=widget.id ",
        "WHERE state.game = $1 AND state.team=$2 ",
        "AND state.state->'solved' != 'null' ",
        "AND widget.config->'type' = '\"prompt\"' ",
        "ORDER BY priority DESC LIMIT 3"
    );

    let stmt = db.prepare_cached(SOLVED).await?;
    let actions = db.query(&stmt, &[&game, &team]).await?;

    actions
        .into_iter()
        .map(|r| {
            let ident = r.try_get(0)?;

            Ok(ident)
        })
        .collect()
}
