use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    expr::{Context, Expr},
    solution::Solution,
    EvalResult,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    style: Style,
    solutions: Vec<Solution>,
    visible: Expr,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State {
    solved: Option<SolutionDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    #[serde(flatten)]
    style: Style,
    disabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Style {
    name: String,
    details: Vec<String>,
    prompt: String,
    submit_button: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SolutionDetails {
    time: OffsetDateTime,
    canonical_text: String,
}

impl Config {
    pub fn default_state(&self) -> State {
        State::default()
    }

    pub fn resolve(&self, state: &State, path: &[&str], mut context: Context) -> EvalResult {
        match path {
            &["solved"] => Ok(state.solved.as_ref().map(|s| s.time).into()),
            &["visible"] => context.eval(&self.visible),
            _ => Err(context.unknown_path(path)),
        }
    }
}

pub fn dummy(visible: &str, solved: Option<OffsetDateTime>) -> (Config, State) {
    (
        Config {
            style: Style::default(),
            solutions: vec![],
            visible: Expr(String::from(visible)),
        },
        State {
            solved: solved.map(|time| SolutionDetails {
                time,
                canonical_text: String::default(),
            }),
        },
    )
}
