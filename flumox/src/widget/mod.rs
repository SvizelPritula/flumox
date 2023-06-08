use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{error::StateMismatchError, expr::Context, EvalResult};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Instance {
    Prompt(prompt::Config, prompt::State),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn resolve(&self, path: &[&str], context: Context) -> EvalResult {
        match self {
            Instance::Prompt(c, s) => c.resolve(s, path, context),
        }
    }
}

pub fn dummy(visible: &str, solved: Option<OffsetDateTime>) -> Instance {
    let (config, state) = prompt::dummy(visible, solved);
    Instance::Prompt(config, state)
}
