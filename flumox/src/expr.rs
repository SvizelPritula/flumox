use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time_expr::{eval, EvalError, Resolve, Value};

use crate::game::GameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Expr(pub String);

impl Default for Expr {
    fn default() -> Self {
        Self(String::from("never"))
    }
}

impl<S> From<S> for Expr
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Expr(value.into())
    }
}

pub type EvalResult = Result<Value, EvalError>;

#[derive(Debug)]
enum EvaluationState {
    Evaluating,
    Evaluated(Value),
}

#[derive(Debug)]
pub struct Context<'a> {
    cache: &'a mut HashMap<String, EvaluationState>,
    pub this: Option<&'a str>,
    pub game: &'a GameState,
}

#[derive(Debug, Default)]
pub struct Cache(HashMap<String, EvaluationState>);

impl<'a> Resolve for Context<'a> {
    fn resolve(&mut self, path: &[&str]) -> EvalResult {
        let Some((module, subpath)) = path.split_first() else {
            return Err(EvalError::UnknownPath);
        };

        let module = match (module, self.this) {
            (&"this", Some(this)) => this,
            (other, _) => other,
        };

        let mut key = module.to_owned();

        for module in subpath {
            key.push('.');
            key.push_str(module);
        }

        match self.cache.get(&key) {
            Some(EvaluationState::Evaluating) => Err(EvalError::CircularDependency),
            Some(EvaluationState::Evaluated(value)) => Ok(*value),
            None => {
                self.cache.insert(key.clone(), EvaluationState::Evaluating);

                let result = self.resolve_raw(module, subpath);

                if let Ok(value) = result {
                    self.cache.insert(key, EvaluationState::Evaluated(value));
                }

                result
            }
        }
    }
}

impl<'a> Context<'a> {
    fn resolve_raw(&mut self, module: &str, path: &[&str]) -> EvalResult {
        let instance = self
            .game
            .instances
            .get(module)
            .ok_or(EvalError::UnknownPath)?;

        let context = Context {
            cache: self.cache,
            this: Some(module),
            game: self.game,
        };

        instance.resolve(path, context)
    }

    pub fn eval(&mut self, expr: &Expr) -> EvalResult {
        eval(&expr.0, self)
    }

    pub fn new(game: &'a GameState, cache: &'a mut Cache) -> Self {
        Context {
            cache: &mut cache.0,
            this: None,
            game,
        }
    }
}
