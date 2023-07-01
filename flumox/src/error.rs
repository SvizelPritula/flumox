use thiserror::Error;
use time_expr::{EvalError, Value};

use crate::action::ActionEffect;

#[derive(Debug, Clone, Copy, Error, Default)]
#[error("state type does't match instance type")]
pub struct StateMismatchError;

#[derive(Debug, Clone, Error)]
pub enum ActionError {
    #[error("this widget does not exist")]
    UnknownIdent,
    #[error("action not allowed for widget")]
    WidgetMismatch,
    #[error("action cannot be currently sent to widget")]
    NotPossible,
    #[error(transparent)]
    Eval(#[from] EvalError),
}

pub type ActionResult<S> = Result<ActionEffect<S>, ActionError>;

pub type EvalResult = Result<Value, EvalError>;

pub type ViewResult<V> = Result<Option<V>, EvalError>;
