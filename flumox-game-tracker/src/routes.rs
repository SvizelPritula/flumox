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
    db::{self, ActionInfo, RecentActionInfo, StateKey},
    error::InternalError,
    parts::{action_description, datetime, not_found, page, short_time, table_style, time_script},
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

    let teams = db::teams(&mut client, path.game).await?;
    let prompts = db::prompts(&mut client, path.game).await?;
    let prompt_states = db::prompt_states(&mut client, path.game).await?;
    let actions = db::recent_actions(&mut client, path.game).await?;

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
                table {
                    thead {
                        tr {
                            th;

                            @for team in &teams {
                                th {
                                    a href={"/" (path.game) "/" (team.id) "/"} { (team.name) }
                                }
                            }
                        }
                    }

                    tbody {
                        @for prompt in &prompts {
                            tr {
                                th {
                                    (prompt.config.style.name)
                                }

                                @for team in &teams {
                                    @let key = StateKey { team: team.id, widget: prompt.id };

                                    td {
                                        @if let Some(time) = prompt_states.get(&key).and_then(|s| s.solved) {
                                            (short_time(time))
                                        }

                                        @if let Some(hints) = prompt_states.get(&key).map(|s| &s.hints).filter(|h| !h.is_empty()) {
                                            " ("
                                            @for hint in hints {
                                                (hint.chars().next().unwrap_or('?'))
                                            }
                                            ")"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                h2 { "Recent actions" }
                @for a in &actions {
                    (action(a))
                }

                (time_script())
                (table_style())
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
