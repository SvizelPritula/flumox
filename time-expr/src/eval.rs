use std::cmp::{max, min};

use crate::{expr::Expr, EvalError, Value};

pub trait Resolve {
    fn resolve(&mut self, path: &[&str]) -> Result<Value, EvalError>;
}

impl<'a> Expr<'a> {
    pub fn eval<R>(&self, resolver: &mut R) -> Result<Value, EvalError>
    where
        R: Resolve,
    {
        match self {
            Expr::Literal { value } => Ok(*value),
            Expr::Field { path } => resolver.resolve(path),
            Expr::And { left, right } => Ok(max(left.eval(resolver)?, right.eval(resolver)?)),
            Expr::Or { left, right } => Ok(min(left.eval(resolver)?, right.eval(resolver)?)),
            Expr::Add { value, duration } => Ok(value.eval(resolver)? + *duration),
        }
    }
}
