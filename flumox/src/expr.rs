use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time_expr::{eval, EvalError, Resolve, Value};

use crate::{error::EvalResult, game::GameState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Expr(pub String);

impl Default for Expr {
    fn default() -> Self {
        Expr::never()
    }
}

impl Expr {
    pub fn never() -> Self {
        Expr(String::from("never"))
    }

    pub fn always() -> Self {
        Expr(String::from("always"))
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

#[derive(Debug)]
enum EvaluationState {
    Evaluating,
    Evaluated(Value),
}

#[derive(Debug)]
pub struct Environment<'a> {
    cache: &'a mut HashMap<String, EvaluationState>,
    pub this: Option<&'a str>,
    pub game: &'a GameState,
}

#[derive(Debug, Default)]
pub struct Cache(HashMap<String, EvaluationState>);

impl<'a> Resolve for Environment<'a> {
    fn resolve(&mut self, path: &[&str]) -> EvalResult {
        let Some((module, subpath)) = path.split_first() else {
            return Err(EvalError::UnknownPath {
                path: String::new().into(),
            });
        };

        let module = match (module, self.this) {
            (&"this", Some(this)) => this,
            (other, _) => other,
        };

        self.resolve_cached(module, subpath)
    }
}

impl<'a> Environment<'a> {
    fn resolve_cached(&mut self, module: &str, path: &[&str]) -> EvalResult {
        let key = Environment::path_to_string(module, path);

        match self.cache.get(&key) {
            Some(EvaluationState::Evaluating) => {
                Err(EvalError::CircularDependency { path: key.into() })
            }
            Some(EvaluationState::Evaluated(value)) => Ok(*value),
            None => {
                self.cache.insert(key.clone(), EvaluationState::Evaluating);

                let result = self.resolve_raw(module, path, &key);

                if let Ok(value) = result {
                    self.cache.insert(key, EvaluationState::Evaluated(value));
                }

                result
            }
        }
    }

    fn resolve_raw(&mut self, module: &str, path: &[&str], path_str: &str) -> EvalResult {
        let env = Environment {
            cache: self.cache,
            this: Some(module),
            game: self.game,
        };

        if module == "team" {
            self.game.team.resolve(path, env)
        } else {
            let instance = self
                .game
                .instances
                .get(module)
                .ok_or(EvalError::UnknownPath {
                    path: path_str.into(),
                })?;

            instance.resolve(path, env)
        }
    }

    pub fn own(&mut self, path: &[&str]) -> EvalResult {
        self.resolve_cached(self.this.unwrap_or("this"), path)
    }

    pub fn eval(&mut self, expr: &Expr) -> EvalResult {
        eval(&expr.0, self)
    }

    fn path_to_string(module: &str, subpath: &[&str]) -> String {
        let mut string = module.to_owned();

        for module in subpath {
            string.push('.');
            string.push_str(module);
        }

        string
    }

    pub fn unknown_path(&self, path: &[&str]) -> EvalError {
        EvalError::UnknownPath {
            path: Environment::path_to_string(self.this.unwrap_or("this"), path).into(),
        }
    }

    pub fn new(game: &'a GameState, cache: &'a mut Cache, this: &'a str) -> Self {
        Environment {
            cache: &mut cache.0,
            this: Some(this),
            game,
        }
    }
}
