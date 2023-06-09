use thiserror::Error;
use time_expr::{Value, EvalError};

#[derive(Debug, Clone, Copy, Error, Default)]
#[error("state type does't match instance type")]
pub struct StateMismatchError;

pub type EvalResult = Result<Value, EvalError>;

pub type ViewResult<V> = Result<Option<V>, EvalError>;
