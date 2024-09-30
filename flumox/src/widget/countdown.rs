use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time_expr::Value;

use crate::{
    error::ViewResult,
    expr::{Environment, Expr},
    text::Text,
    view_context::ViewContext,
    EvalResult,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    name: Option<String>,
    details: Text,
    time: Expr,
    visible: Expr,
    done_text: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    name: Option<String>,
    details: Vec<String>,
    value: CountdownValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum CountdownValue {
    Unknown,
    Time {
        #[serde(with = "time::serde::rfc3339")]
        time: OffsetDateTime,
    },
    Done {
        text: String,
    },
}

impl Config {
    pub fn default_state(&self) -> State {
        State
    }

    pub fn resolve(&self, _state: &State, path: &[&str], mut env: Environment) -> EvalResult {
        match *path {
            ["visible"] => env.eval(&self.visible),
            ["time"] => env.eval(&self.time),
            _ => Err(env.unknown_path(path)),
        }
    }

    pub fn view(&self, _state: &State, mut ctx: ViewContext) -> ViewResult<View> {
        let visible = ctx.eval(&self.visible)?;

        if !visible {
            return Ok(None);
        }

        let time = ctx.env.eval(&self.time)?;

        let value = match &time {
            Value::Always => CountdownValue::Done {
                text: self.done_text.clone(),
            },
            Value::Since(t) => {
                if ctx.time.after(time) {
                    CountdownValue::Done {
                        text: self.done_text.clone(),
                    }
                } else {
                    CountdownValue::Time { time: *t }
                }
            }
            Value::Never => CountdownValue::Unknown,
        };

        Ok(Some(View {
            name: self.name.clone(),
            details: self.details.render(&mut ctx)?,
            value,
        }))
    }
}

impl View {
    pub fn obsolete(&self) -> bool {
        matches!(self.value, CountdownValue::Done { .. })
    }
}
