use std::collections::HashMap;

use flumox::{Cache, Environment, GameState, TimeTracker, View, ViewContext};
use serde::Serialize;
use time::OffsetDateTime;
use time_expr::EvalError;
use uuid::Uuid;

use crate::types::{InstanceMetadata, WidgetInstance};

pub struct RenderResult {
    pub widgets: Vec<WidgetInstance>,
    pub valid_until: Option<OffsetDateTime>,
}

pub fn render(
    game: &GameState,
    meta: &HashMap<String, InstanceMetadata>,
    time: OffsetDateTime,
) -> Result<RenderResult, EvalError> {
    let mut cache = Cache::default();
    let mut tracker = TimeTracker::new(time);

    let mut result = Vec::new();

    for (ident, widget) in game.instances.iter() {
        let env = Environment::new(game, &mut cache, ident);
        let ctx = ViewContext::new(env, &mut tracker);

        if let Some(view) = widget.view(ctx)? {
            let Some(meta) = meta.get(ident) else {
                return Err(EvalError::UnknownPath { path: ident.clone().into_boxed_str() })
            };

            result.push(WidgetInstance { view, id: meta.id });
        }
    }

    result.sort_by_key(|w| w.view.obsolete());

    Ok(RenderResult {
        widgets: result,
        valid_until: tracker.valid_until(),
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetInstanceDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
    pub id: Uuid,
}

pub fn delta(new: &[WidgetInstance], old: &[WidgetInstance]) -> Vec<WidgetInstanceDelta> {
    let old: HashMap<Uuid, &View> = old
        .iter()
        .map(|WidgetInstance { view, id }| (*id, view))
        .collect();

    let mut delta = Vec::new();

    for WidgetInstance { view, id } in new {
        let view = if old.get(id).copied().is_some_and(|old| old == view) {
            None
        } else {
            Some(view.clone())
        };

        delta.push(WidgetInstanceDelta { view, id: *id });
    }

    delta
}
