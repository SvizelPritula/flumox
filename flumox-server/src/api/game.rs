use axum::Json;
use time::OffsetDateTime;

use crate::{
    db::load_state, error::InternalError, extract::DbConnection, session::Session,
    types::WidgetInstance, view::render,
};

pub async fn view(
    Session { game, team }: Session,
    DbConnection(mut db): DbConnection,
) -> Result<Json<Vec<WidgetInstance>>, InternalError> {
    let time = OffsetDateTime::now_utc();
    let (game, meta) = load_state(&mut db, game, team).await?;

    let result = render(&game, &meta, time)?;

    Ok(Json(result))
}
