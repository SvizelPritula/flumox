use fingerprint_struct::Fingerprint;
use serde::{Deserialize, Serialize};

use crate::{
    error::{StateMismatchError, ViewResult},
    expr::Environment,
    view_context::ViewContext,
    EvalResult,
};

mod prompt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Config {
    Prompt(prompt::Config),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum State {
    Prompt(prompt::State),
}

#[derive(Debug, Clone)]
pub enum Instance {
    Prompt(prompt::Config, prompt::State),
}

#[derive(Debug, Clone, Serialize, Fingerprint)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum View {
    Prompt(prompt::View),
}

impl Config {
    pub fn instance_default(self) -> Instance {
        match self {
            Config::Prompt(config) => {
                let state = config.default_state();
                Instance::Prompt(config, state)
            }
        }
    }

    pub fn instance(self, state: State) -> Result<Instance, StateMismatchError> {
        match (self, state) {
            (Config::Prompt(c), State::Prompt(s)) => Ok(Instance::Prompt(c, s)),
            #[allow(unreachable_patterns)]
            _ => Err(StateMismatchError),
        }
    }
}

impl Instance {
    pub fn resolve(&self, path: &[&str], env: Environment) -> EvalResult {
        match self {
            Instance::Prompt(c, s) => c.resolve(s, path, env),
        }
    }

    pub fn view(&self, ctx: ViewContext) -> ViewResult<View> {
        let view = match self {
            Instance::Prompt(c, s) => c.view(s, ctx)?.map(View::Prompt),
        };

        Ok(view)
    }
}
