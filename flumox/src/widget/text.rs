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
    #[serde(default = "Expr::never")]
    obsolete: Expr,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    heading: Option<String>,
    content: Vec<String>,
    #[serde(skip)]
    obsolete: bool,
}

impl Config {
    pub fn default_state(&self) -> State {
        State
    }

    pub fn resolve(&self, _state: &State, path: &[&str], mut env: Environment) -> EvalResult {
        match *path {
            ["visible"] => env.eval(&self.visible),
            ["obsolete"] => env.eval(&self.visible),
            _ => Err(env.unknown_path(path)),
        }
    }

    pub fn view(&self, _state: &State, mut ctx: ViewContext) -> ViewResult<View> {
        let visible = ctx.eval(&self.visible)?;
        let obsolete = ctx.eval(&self.obsolete)?;

        if visible {
            Ok(Some(View {
                heading: self.heading.clone(),
                content: self.content.clone(),
                obsolete,
            }))
        } else {
            Ok(None)
        }
    }
}

impl View {
    pub fn obsolete(&self) -> bool {
        self.obsolete
    }
}
