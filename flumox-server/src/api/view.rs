use axum::Json;
use flumox::View;
use time::OffsetDateTime;

use crate::{db::load_state, error::InternalError, extract::DbConnection, session::Session};

pub async fn view(
    Session { game, team }: Session,
    DbConnection(mut db): DbConnection,
) -> Result<Json<Vec<View>>, InternalError> {
    let time = OffsetDateTime::now_utc();
    let game = load_state(&mut db, game, team).await?;

    let result = game.view(time)?;

    Ok(Json(result))
}
