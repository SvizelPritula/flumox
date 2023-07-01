use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{expr::Expr, Environment, EvalResult};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Attributes {
    map: HashMap<String, Expr>,
}

impl Attributes {
    pub fn resolve(&self, path: &[&str], mut env: Environment) -> EvalResult {
        let [id] = *path else {
            return Err(env.unknown_path(path));
        };

        let Some(expr) = self.map.get(id) else {
            return Err(env.unknown_path(path));
        };

        env.eval(expr)
    }
}
