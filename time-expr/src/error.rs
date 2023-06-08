use std::fmt::{self, Display};

use thiserror::Error;

use crate::parse::TokenType;

#[derive(Debug, Clone, Error)]
pub enum EvalError {
    #[error("unknown path \"{path}\"")]
    UnknownPath { path: Box<str> },
    #[error("circular dependency: \"{path}\" has itself as a dependency")]
    CircularDependency { path: Box<str> },

    #[error("unexpected char '{char}'", char=.char.escape_default())]
    UnknownChar { char: char },
    #[error("unexpected {token} at {pos}", pos=PosOrEnd(*.pos))]
    UnexpectedToken {
        token: TokenType,
        pos: Option<usize>,
    },
    #[error("literal out of range at {pos}")]
    LiteralOutOfRange { pos: usize },
    #[error("unknown unit \"{unit}\"")]
    UnknownUnit { unit: Box<str> },
}

struct PosOrEnd(Option<usize>);

impl Display for PosOrEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(pos) => pos.fmt(f),
            None => write!(f, "end"),
        }
    }
}
