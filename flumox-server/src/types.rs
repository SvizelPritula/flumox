use flumox::View;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct GameInfo {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamInfo {
    pub name: String,
    pub game: GameInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TeamId {
    pub game: Uuid,
    pub team: Uuid,
}

#[derive(Debug, Clone)]
pub struct InstanceMetadata {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetInstance {
    #[serde(flatten)]
    pub view: View,
    pub id: Uuid,
}
