use deadpool_postgres::Client;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{error::InternalError, session::SessionToken, types::TeamInfo};

const GET_TEAM_BY_KEY: &str = "SELECT game, id, name FROM team WHERE access_code=$1";
const CREATE_SESSION: &str =
    "INSERT INTO session (id, game, team, token, created) VALUES ($1, $2, $3, $4, $5)";

pub async fn login(
    db: &mut Client,
    code: &str,
) -> Result<Option<(SessionToken, TeamInfo)>, InternalError> {
    let db = db.transaction().await?;

    let row = db.query_opt(GET_TEAM_BY_KEY, &[&code]).await?;

    let Some(row) = row else { return Ok(None); };

    let game: Uuid = row.try_get(0)?;
    let team: Uuid = row.try_get(1)?;
    let name: String = row.try_get(2)?;

    let id = Uuid::new_v4();
    let token = SessionToken::new();
    let time = OffsetDateTime::now_utc();

    db.execute(CREATE_SESSION, &[&id, &game, &team, &token.0, &time])
        .await?;

    db.commit().await?;

    let info = TeamInfo { name };

    Ok(Some((token, info)))
}
