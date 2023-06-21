use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GameInfo {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamInfo {
    pub name: String,
    pub game: GameInfo,
}
