use crate::{widget::Instance, Cache, Environment, TimeTracker, View, ViewContext};
use indexmap::IndexMap;
use time::OffsetDateTime;
use time_expr::EvalError;

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub instances: IndexMap<String, Instance>,
}

impl GameState {
    pub fn view(&self, time: OffsetDateTime) -> Result<Vec<View>, EvalError> {
        let mut cache = Cache::default();
        let mut tracker = TimeTracker::new(time);

        let mut result = Vec::new();

        for (ident, widget) in self.instances.iter() {
            let env = Environment::new(self, &mut cache, ident);
            let ctx = ViewContext::new(env, &mut tracker);

            if let Some(view) = widget.view(ctx)? {
                result.push(view);
            }
        }

        Ok(result)
    }
}
