use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time_expr::EvalError;

use crate::{error::ActionResult, expr::Expr, ActionError, Environment, Instance, State, Toast};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Action {
    Answer(Answer),
    Hint(Hint),
}

impl Instance {
    pub fn submit(&self, action: &Action, ctx: ActionContext) -> ActionResult<State> {
        match (self, action) {
            (Instance::Prompt(config, state), Action::Answer(action)) => config
                .submit_answer(state, action, ctx)
                .map(|e| e.map(Box::new).map(State::Prompt)),
            (Instance::Prompt(config, state), Action::Hint(action)) => config
                .take_hint(state, action, ctx)
                .map(|e| e.map(Box::new).map(State::Prompt)),
            _ => Err(ActionError::WidgetMismatch),
        }
    }
}

#[derive(Debug)]
pub struct ActionEffect<S> {
    pub new_state: Option<S>,
    pub toast: Option<Toast>,
}

impl<S> ActionEffect<S> {
    pub fn map<N, F>(self, op: F) -> ActionEffect<N>
    where
        F: FnOnce(S) -> N,
    {
        ActionEffect {
            new_state: self.new_state.map(op),
            toast: self.toast,
        }
    }

    pub fn new(state: Option<S>, toast: Option<Toast>) -> Self {
        Self {
            new_state: state,
            toast,
        }
    }

    pub fn with_toast(toast: Toast) -> Self {
        Self {
            toast: Some(toast),
            new_state: None,
        }
    }
}

#[derive(Debug)]
pub struct ActionContext<'a> {
    pub env: Environment<'a>,
    pub time: OffsetDateTime,
}

impl<'a> ActionContext<'a> {
    pub fn eval(&mut self, expr: &Expr) -> Result<bool, EvalError> {
        let value = self.env.eval(expr)?;
        Ok(value.to_bool(self.time))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub ident: String,
}
