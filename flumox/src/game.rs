use crate::{attributes::Attributes, widget::Instance};
use indexmap::IndexMap;

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub instances: IndexMap<String, Instance>,
    pub team: Attributes,
}
