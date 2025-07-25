use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use time_expr::{EvalError, Value};

use crate::{
    action::{ActionContext, ActionEffect, Answer, Hint},
    error::{ActionResult, ViewResult},
    expr::{Environment, Expr},
    solution::Solution,
    text::Text,
    toast::Message,
    view_context::ViewContext,
    ActionError, EvalResult, Instance, Toast, ToastType,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub style: Style,
    details: Text,
    solutions: Vec<Solution>,
    visible: Expr,
    #[serde(default = "Expr::never")]
    disabled: Expr,
    #[serde(default)]
    hints: Vec<HintConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    on_solution_correct: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    on_solution_incorrect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution_exclusion_group: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State {
    pub solved: Option<SolutionDetails>,
    pub hints: HashMap<String, OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    #[serde(flatten)]
    style: Style,
    details: Vec<String>,
    disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution: Option<String>,
    hints: Vec<HintView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<TimeView>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
    pub name: String,
    prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    submit_button: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionDetails {
    #[serde(with = "time::serde::rfc3339")]
    pub time: OffsetDateTime,
    pub canonical_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HintConfig {
    ident: String,
    name: String,
    content: Vec<String>,
    available: Expr,
    #[serde(default = "Expr::always")]
    visible: Expr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    take_button: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    on_hint_taken: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct HintView {
    ident: String,
    name: String,
    #[serde(flatten)]
    state: HintStateView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "state")]
enum HintStateView {
    Unknown,
    Future {
        #[serde(with = "time::serde::rfc3339")]
        time: OffsetDateTime,
    },
    Available {
        #[serde(skip_serializing_if = "Option::is_none")]
        button: Option<String>,
    },
    Taken {
        content: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum TimeView {
    Solving {
        #[serde(with = "time::serde::rfc3339")]
        since: OffsetDateTime,
    },
    Solved {
        after: Duration,
    },
}

impl Config {
    pub fn default_state(&self) -> State {
        State::default()
    }

    pub fn resolve(&self, state: &State, path: &[&str], mut env: Environment) -> EvalResult {
        match *path {
            ["solved"] => Ok(state.solved.as_ref().map(|s| s.time).into()),
            ["visible"] => env.eval(&self.visible),
            ["disabled"] => env.eval(&self.disabled),
            ["hint", hint, "available"] => self
                .hints
                .iter()
                .find(|h| h.ident == hint)
                .ok_or_else(|| env.unknown_path(path))
                .and_then(|h| env.eval(&h.available)),
            ["hint", hint, "visible"] => self
                .hints
                .iter()
                .find(|h| h.ident == hint)
                .ok_or_else(|| env.unknown_path(path))
                .and_then(|h| env.eval(&h.visible)),
            ["hint", hint, "taken"] => state
                .hints
                .get(hint)
                .map(|time| Ok(Value::Since(*time)))
                .unwrap_or(Ok(Value::Never)),
            _ => Err(env.unknown_path(path)),
        }
    }

    pub fn view(&self, state: &State, mut ctx: ViewContext) -> ViewResult<View> {
        let visible_since = ctx.env.eval(&self.visible)?;
        let visible = ctx.time.after(visible_since);
        let disabled = ctx.eval(&self.disabled)?;

        if !visible {
            return Ok(None);
        }

        let solved = state.solved.is_some();

        let mut hints = Vec::new();

        for hint in &self.hints {
            if !solved && !ctx.eval(&hint.visible)? {
                continue;
            }

            let state = if solved || state.hints.contains_key(&hint.ident) {
                HintStateView::Taken {
                    content: hint.content.clone(),
                }
            } else {
                let available_time = ctx.env.eval(&hint.available)?;

                match available_time {
                    Value::Always => HintStateView::Available {
                        button: hint.take_button.to_owned(),
                    },
                    Value::Since(time) => {
                        if ctx.time.after(available_time) {
                            HintStateView::Available {
                                button: hint.take_button.to_owned(),
                            }
                        } else {
                            HintStateView::Future { time }
                        }
                    }
                    Value::Never => HintStateView::Unknown,
                }
            };

            hints.push(HintView {
                ident: hint.ident.to_owned(),
                name: hint.name.to_owned(),
                state,
            });
        }

        let time = match (visible_since, &state.solved) {
            (Value::Always | Value::Never, _) => None,
            (Value::Since(since), None) => Some(TimeView::Solving { since }),
            (Value::Since(since), Some(SolutionDetails { time: until, .. })) => {
                Some(TimeView::Solved {
                    after: *until - since,
                })
            }
        };

        Ok(Some(View {
            style: self.style.clone(),
            details: self.details.render(&mut ctx)?,
            disabled: solved | disabled,
            solution: state.solved.as_ref().map(|s| s.canonical_text.clone()),
            hints,
            time,
        }))
    }

    fn active(&self, state: &State, ctx: &mut ActionContext) -> Result<bool, EvalError> {
        let visible = ctx.eval(&self.visible)?;
        let disabled = ctx.eval(&self.disabled)?;

        Ok(visible & state.solved.is_none() & !disabled)
    }

    pub fn submit_answer(
        &self,
        state: &State,
        action: &Answer,
        mut ctx: ActionContext,
    ) -> ActionResult<State> {
        if !self.active(state, &mut ctx)? {
            return Err(ActionError::NotPossible);
        }

        let mut banned = HashSet::new();

        if let Some(group) = self.solution_exclusion_group.as_ref() {
            for instance in ctx.env.game.instances.values() {
                if let Instance::Prompt(config, state) = instance {
                    if config
                        .solution_exclusion_group
                        .as_ref()
                        .is_some_and(|g| g == group)
                    {
                        if let Some(solution) = &state.solved {
                            banned.insert(solution.canonical_text.as_str());
                        }
                    }
                }
            }
        }

        if let Some(solution) = self.solutions.iter().find(|s| s.check(&action.answer)) {
            let canonical_text = solution
                .canonical_answer()
                .unwrap_or_else(|| action.answer.to_owned());

            if banned.contains(canonical_text.as_str()) {
                return Ok(ActionEffect::with_toast(Toast {
                    message: self
                        .on_solution_incorrect
                        .clone()
                        .map(Message::Custom)
                        .unwrap_or(Message::SolutionIncorrect),
                    class: ToastType::Danger,
                }));
            }

            let mut state = state.clone();

            state.solved = Some(SolutionDetails {
                time: ctx.time,
                canonical_text,
            });

            Ok(ActionEffect::new(
                Some(state),
                Some(Toast {
                    message: self
                        .on_solution_correct
                        .clone()
                        .map(Message::Custom)
                        .unwrap_or(Message::SolutionCorrect),
                    class: ToastType::Success,
                }),
            ))
        } else {
            return Ok(ActionEffect::with_toast(Toast {
                message: self
                    .on_solution_incorrect
                    .clone()
                    .map(Message::Custom)
                    .unwrap_or(Message::SolutionIncorrect),
                class: ToastType::Danger,
            }));
        }
    }

    pub fn take_hint(
        &self,
        state: &State,
        action: &Hint,
        mut ctx: ActionContext,
    ) -> ActionResult<State> {
        if !self.active(state, &mut ctx)? {
            return Err(ActionError::NotPossible);
        }

        let ident = &action.ident;

        let Some(hint) = self.hints.iter().find(|h| *h.ident == *ident) else {
            return Err(ActionError::UnknownIdent);
        };

        let visible = ctx.env.eval(&hint.visible)?;
        let visible = visible.to_bool(ctx.time);

        let available = ctx.env.eval(&hint.available)?;
        let available = available.to_bool(ctx.time);

        if !visible || !available {
            return Err(ActionError::NotPossible);
        }

        if state.hints.contains_key(ident) {
            return Err(ActionError::NotPossible);
        }

        let mut state = state.clone();

        state.hints.insert(ident.to_owned(), ctx.time);

        Ok(ActionEffect::new(
            Some(state),
            Some(Toast {
                message: hint
                    .on_hint_taken
                    .clone()
                    .map(Message::Custom)
                    .unwrap_or(Message::HintTaken),
                class: ToastType::Success,
            }),
        ))
    }
}

impl View {
    pub fn obsolete(&self) -> bool {
        self.disabled
    }
}
