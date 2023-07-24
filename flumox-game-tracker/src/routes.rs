use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use deadpool_postgres::Pool;
use flumox::Instance;
use maud::{html, Markup};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{self, last_solved, ActionInfo, RecentActionInfo},
    error::InternalError,
    parts::{action_description, datetime, not_found, page, time_script},
};

pub async fn root(State(pool): State<Pool>) -> Result<Markup, InternalError> {
    let mut client = pool.get().await?;
    let mut client = client.transaction().await?;

    let games = db::games(&mut client).await?;

    Ok(page(
        "Games",
        html!(
            h1 { "Flumox" }
            h2 { "Games" }
            @for game in games {
                p {
                    a href={"/" (game.id) "/"} { (game.name) }
                }
            }
        ),
    ))
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct GamePath {
    game: Uuid,
}

pub async fn game(
    State(pool): State<Pool>,
    Path(path): Path<GamePath>,
) -> Result<(StatusCode, Markup), InternalError> {
    let mut client = pool.get().await?;
    let mut client = client.transaction().await?;

    let game = db::game_name(&mut client, path.game).await?;

    let Some(game) = game else {
        return Ok(not_found("Game"));
    };

    let teams_simple = db::teams(&mut client, path.game).await?;
    let actions = db::recent_actions(&mut client, path.game).await?;

    let mut teams = Vec::new();

    for team in teams_simple {
        let solved = last_solved(&mut client, path.game, team.id).await?;
        teams.push((team, solved))
    }

    fn action(action: &RecentActionInfo) -> Markup {
        html!(
            p {
                (datetime(action.time))
                " "
                b { (action.team) }
                " - "
                b { (action.widget) }
                ": "
                (action_description(&action.payload))
            }
        )
    }

    Ok((
        StatusCode::OK,
        page(
            &game,
            html!(
                h1 { (&game) }

                h2 { "Teams" }
                @for (team, solved) in teams {
                    p {
                        a href={"/" (path.game) "/" (team.id) "/"} { (team.name) }

                        " ("
                        @for (i, widget) in solved.iter().enumerate() {
                            @if i != 0 {
                                ", "
                            }

                            b { (widget) }
                        }
                        ")"
                    }
                }

                h2 { "Recent actions" }
                @for a in &actions {
                    (action(a))
                }

                {(time_script())}
            ),
        ),
    ))
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TeamPath {
    game: Uuid,
    team: Uuid,
}

pub async fn team(
    State(pool): State<Pool>,
    Path(path): Path<TeamPath>,
) -> Result<(StatusCode, Markup), InternalError> {
    let mut client = pool.get().await?;
    let mut client = client.transaction().await?;

    let team = db::team_name(&mut client, path.game, path.team).await?;

    let Some(team) = team else {
        return Ok(not_found("Team"));
    };

    let widgets = db::states(&mut client, path.game, path.team).await?;
    let actions = db::actions(&mut client, path.game, path.team).await?;

    fn widget(state: &Instance, ident: &str) -> Option<Markup> {
        match state {
            Instance::Prompt(config, state) => Some(html!(
                h3 { (config.style.name) " (" (ident) ")" }
                p {
                    @match state.solved.as_ref() {
                        Some(details) => "Solved at " i { (datetime(details.time)) },
                        None => i { "Not solved" },
                    }
                }
                p { "Hints taken:" }
                @if !state.hints.is_empty() {
                    ul {
                        @for hint in state.hints.iter() {
                            li {
                                b { (hint.0) }
                                " at "
                                i { (datetime(*hint.1)) }
                            }
                        }
                    }
                } @else {
                    p { i { "None" } }
                }
            )),
            _ => None,
        }
    }

    fn action(action: &ActionInfo) -> Markup {
        html!(
            p {
                (datetime(action.time))
                " "
                b { (action.widget) }
                ": "
                (action_description(&action.payload))
            }
        )
    }

    Ok((
        StatusCode::OK,
        page(
            &team,
            html!(
                h1 { (&team) }

                h2 { "State" }
                @for w in &widgets {
                    @if let Some(state) = widget(&w.instance, &w.ident) {
                        (state)
                    }
                }

                h2 { "Actions" }
                @for a in &actions {
                    (action(a))
                }
                @if actions.is_empty() {
                    p { i { "None" } }
                }

                {(time_script())}
            ),
        ),
    ))
}
