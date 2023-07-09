use serde::{Deserialize, Serialize};

use crate::{
    error::{StateMismatchError, ViewResult},
    expr::Environment,
    view_context::ViewContext,
    EvalResult,
};

mod countdown;
mod prompt;
mod text;

macro_rules! define_widgets {
    ($($type: ident, $module: ident);*) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case", tag = "type")]
        pub enum Config {
            $(
                $type($module::Config),
            )*
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case", tag = "type")]
        pub enum State {
            $(
                $type($module::State),
            )*
        }

        #[derive(Debug, Clone)]
        pub enum Instance {
            $(
                $type($module::Config, $module::State),
            )*
        }

        #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
        #[serde(rename_all = "kebab-case", tag = "type")]
        pub enum View {
            $(
                $type($module::View),
            )*
        }

        impl Config {
            pub fn instance_default(self) -> Instance {
                match self {
                    $(
                        Config::$type(config) => {
                            let state = config.default_state();
                            Instance::$type(config, state)
                        }
                    )*
                }
            }

            pub fn instance(self, state: State) -> Result<Instance, StateMismatchError> {
                match (self, state) {
                    $(
                        (Config::$type(c), State::$type(s)) => Ok(Instance::$type(c, s)),
                    )*
                    #[allow(unreachable_patterns)]
                    _ => Err(StateMismatchError),
                }
            }
        }

        impl Instance {
            pub fn resolve(&self, path: &[&str], env: Environment) -> EvalResult {
                match self {
                    $(
                        Instance::$type(c, s) => c.resolve(s, path, env),
                    )*
                }
            }

            pub fn view(&self, ctx: ViewContext) -> ViewResult<View> {
                let view = match self {
                    $(
                        Instance::$type(c, s) => c.view(s, ctx)?.map(View::$type),
                    )*
                };

                Ok(view)
            }
        }

        impl View {
            pub fn obsolete(&self) -> bool {
                match self {
                    $(
                        View::$type(v) => v.obsolete(),
                    )*
                }
            }
        }
    };
}

define_widgets!(
    Prompt, prompt;
    Text, text;
    Countdown, countdown
);
