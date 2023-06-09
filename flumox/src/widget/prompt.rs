use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    error::ViewResult,
    expr::{Environment, Expr},
    solution::Solution,
    view_context::ViewContext,
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
    #[serde(with = "time::serde::rfc3339")]
    time: OffsetDateTime,
    canonical_text: String,
}

impl Config {
    pub fn default_state(&self) -> State {
        State::default()
    }

    pub fn resolve(&self, state: &State, path: &[&str], mut env: Environment) -> EvalResult {
        match path {
            &["solved"] => Ok(state.solved.as_ref().map(|s| s.time).into()),
            &["visible"] => env.eval(&self.visible),
            _ => Err(env.unknown_path(path)),
        }
    }

    pub fn view(&self, state: &State, mut ctx: ViewContext) -> ViewResult<View> {
        let visible = ctx.env.own(&["visible"])?;
        let visible = ctx.time.if_after(visible);

        if !visible {
            return Ok(None);
        }

        Ok(Some(View {
            style: self.style.clone(),
            disabled: state.solved.is_some(),
        }))
    }
}
