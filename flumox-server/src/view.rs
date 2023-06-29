use std::collections::HashMap;

use flumox::{Cache, Environment, GameState, TimeTracker, ViewContext};
use time::OffsetDateTime;
use time_expr::EvalError;

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

    Ok(RenderResult {
        widgets: result,
        valid_until: tracker.valid_until(),
    })
}
