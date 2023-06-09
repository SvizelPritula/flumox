use axum::Json;
use flumox::View;
use time::OffsetDateTime;
use uuid::{uuid, Uuid};

use crate::{db::load_state, error::InternalError, extract::DbConnection};

mod session;

pub use session::{login, me};

const GAME: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
const TEAM: Uuid = uuid!("00000000-0000-0000-0000-000000000000");

pub async fn debug(DbConnection(mut db): DbConnection) -> Result<Json<Vec<View>>, InternalError> {
    let time = OffsetDateTime::now_utc();

    let game = load_state(&mut db, GAME, TEAM).await?;

    let result = game.view(time)?;

    Ok(Json(result))
}
