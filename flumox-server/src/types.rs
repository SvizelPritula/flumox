use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TeamInfo {
    pub name: String,
}
