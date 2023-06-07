use time::Duration;

use crate::Value;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Literal {
        value: Value,
    },
    Field {
        path: Vec<&'a str>,
    },
    And {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
    Or {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
    Add {
        value: Box<Expr<'a>>,
        duration: Duration,
    },
}
