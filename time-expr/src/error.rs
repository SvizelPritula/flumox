use thiserror::Error;

use crate::parse::TokenType;

#[derive(Debug, Clone, Error)]
pub enum EvalError {
    #[error("unknown path")]
    UnknownPath,
    #[error("circular dependency")]
    CircularDependency,

    #[error("unexpected char '{char}'", char=char.escape_default())]
    UnknownChar { char: char },
    #[error("unexpected {token}")]
    UnexpectedToken { token: TokenType },
    #[error("literal out of range")]
    LiteralOutOfRange,
    #[error("unknown unit")]
    UnknownUnit,
}
