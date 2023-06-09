use axum::Json;
use flumox::{Cache, Environment, TimeTracker, View, ViewContext};
use time::OffsetDateTime;
use uuid::{uuid, Uuid};

use crate::{db::load_state, error::InternalError, extract::DbConnection};

const GAME: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
const TEAM: Uuid = uuid!("00000000-0000-0000-0000-000000000000");

pub async fn debug(DbConnection(mut db): DbConnection) -> Result<Json<Vec<View>>, InternalError> {
    let time = OffsetDateTime::now_utc();

    let mut db = db.transaction().await?;
    let game = load_state(&mut db, GAME, TEAM).await?;

    let mut cache = Cache::default();
    let mut tracker = TimeTracker::new(time);

    let mut result = Vec::new();

    for (ident, widget) in game.instances.iter() {
        let env = Environment::new(&game, &mut cache, ident);
        let ctx = ViewContext::new(env, &mut tracker);

        if let Some(view) = widget.view(ctx)? {
            result.push(view);
        }
    }

    Ok(Json(result))
}
