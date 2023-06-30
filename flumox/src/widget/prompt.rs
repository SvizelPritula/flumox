use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    action::{ActionContext, ActionEffect, Answer},
    error::{ActionResult, ViewResult},
    expr::{Environment, Expr},
    solution::Solution,
    view_context::ViewContext,
    ActionError, EvalResult, Toast, ToastType,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    #[serde(flatten)]
    style: Style,
    disabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        match *path {
            ["solved"] => Ok(state.solved.as_ref().map(|s| s.time).into()),
            ["visible"] => env.eval(&self.visible),
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

    pub fn submit_answer(
        &self,
        state: &State,
        action: &Answer,
        mut ctx: ActionContext,
    ) -> ActionResult<State> {
        let visible = ctx.env.own(&["visible"])?;
        let visible = visible.to_bool(ctx.time);
        let active = visible & state.solved.is_none();

        if !active {
            return Err(ActionError::NotPossible);
        }

        if let Some(solution) = self.solutions.iter().find(|s| s.check(&action.answer)) {
            let mut state = state.clone();

            state.solved = Some(SolutionDetails {
                time: ctx.time,
                canonical_text: solution.to_string(),
            });

            Ok(ActionEffect::new(
                Some(state),
                Some(Toast::new(
                    "Solution correct".to_owned(),
                    ToastType::Success,
                )),
            ))
        } else {
            Ok(ActionEffect::with_toast(Toast::new(
                "Solution incorrect".to_owned(),
                ToastType::Danger,
            )))
        }
    }
}
