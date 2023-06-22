use std::collections::HashMap;

use deadpool_postgres::Client;
use flumox::{Config, GameState};
use indexmap::IndexMap;
use tokio_postgres::types::Json;
use uuid::Uuid;

use crate::{error::InternalError, types::InstanceMetadata};

pub async fn load_state(
    db: &mut Client,
    game: Uuid,
    team: Uuid,
) -> Result<(GameState, HashMap<String, InstanceMetadata>), InternalError> {
    const LOAD_STATE: &str = concat!(
        "SELECT widget.id, widget.ident, widget.config, state.state ",
        "FROM widget LEFT JOIN state ",
        "ON state.game=widget.game AND state.widget=widget.id AND state.team=$2 ",
        "WHERE widget.game=$1 ",
        "ORDER BY widget.id"
    );

    let statement = db.prepare_cached(LOAD_STATE).await?;
    let rows = db.query(&statement, &[&game, &team]).await?;

    let mut instances = IndexMap::new();
    let mut metadata = HashMap::new();

    for row in rows {
        let id: Uuid = row.try_get(0)?;
        let ident: String = row.try_get(1)?;
        let Json(config): Json<Config> = row.try_get(2)?;

        let instance = if let Some(Json(state)) = row.try_get(3)? {
            config.instance(state)?
        } else {
            config.instance_default()
        };

        instances.insert(ident.clone(), instance);
        metadata.insert(ident, InstanceMetadata { id });
    }

    Ok((GameState { instances }, metadata))
}
