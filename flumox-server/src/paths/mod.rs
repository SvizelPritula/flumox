use axum::Json;
use flumox::Instance;
use uuid::{uuid, Uuid};

use crate::{db::load_state, error::InternalError, extract::DbConnection};

const GAME: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
const TEAM: Uuid = uuid!("00000000-0000-0000-0000-000000000000");

pub async fn debug(
    DbConnection(mut db): DbConnection,
) -> Result<Json<Vec<(String, Instance)>>, InternalError> {
    let mut db = db.transaction().await?;
    let state = load_state(&mut db, GAME, TEAM).await?;

    let result = state.instances.into_iter().collect();

    Ok(Json(result))
}
