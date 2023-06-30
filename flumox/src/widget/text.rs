use serde::{Deserialize, Serialize};

use crate::{
    error::ViewResult,
    expr::{Environment, Expr},
    view_context::ViewContext,
    EvalResult,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    heading: Option<String>,
    content: Vec<String>,
    visible: Expr,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    heading: Option<String>,
    content: Vec<String>,
}

impl Config {
    pub fn default_state(&self) -> State {
        State::default()
    }

    pub fn resolve(&self, _state: &State, path: &[&str], mut env: Environment) -> EvalResult {
        match *path {
            ["visible"] => env.eval(&self.visible),
            _ => Err(env.unknown_path(path)),
        }
    }

    pub fn view(&self, _state: &State, mut ctx: ViewContext) -> ViewResult<View> {
        let visible = ctx.env.own(&["visible"])?;
        let visible = ctx.time.if_after(visible);

        if visible {
            Ok(Some(View {
                heading: self.heading.clone(),
                content: self.content.clone(),
            }))
        } else {
            Ok(None)
        }
    }
}
