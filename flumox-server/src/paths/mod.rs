use axum::Json;
use flumox::{Cache, Context};
use time_expr::{Resolve, Value};
use uuid::{uuid, Uuid};

use crate::{db::load_state, error::InternalError, extract::DbConnection};

const GAME: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
const TEAM: Uuid = uuid!("00000000-0000-0000-0000-000000000000");

pub async fn debug(DbConnection(mut db): DbConnection) -> Result<Json<Vec<String>>, InternalError> {
    let mut db = db.transaction().await?;
    let game = load_state(&mut db, GAME, TEAM).await?;

    let mut cache = Cache::default();
    let mut context = Context::new(&game, &mut cache);

    let mut result = Vec::new();

    for module in game.instances.keys() {
        match context.resolve(&[module, "visible"])? {
            Value::Always => result.push(format!("always")),
            Value::Since(time) => result.push(format!("{time}")),
            Value::Never => result.push(format!("never")),
        }
    }

    Ok(Json(result))
}
