use deadpool_postgres::Client;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    error::InternalError,
    session::{Session, SessionToken},
    types::TeamInfo,
};

pub async fn login(
    db: &mut Client,
    code: &str,
) -> Result<Option<(SessionToken, TeamInfo)>, InternalError> {
    const TEAM_BY_KEY: &str = "SELECT game, id, name FROM team WHERE access_code=$1";
    const CREATE_SESSION: &str =
        "INSERT INTO session (id, game, team, token, created) VALUES ($1, $2, $3, $4, $5)";

    let db = db.transaction().await?;

    let statement = db.prepare_cached(TEAM_BY_KEY).await?;
    let row = db.query_opt(&statement, &[&code]).await?;

    let Some(row) = row else { return Ok(None); };

    let game: Uuid = row.try_get(0)?;
    let team: Uuid = row.try_get(1)?;
    let name: String = row.try_get(2)?;

    let id = Uuid::new_v4();
    let token = SessionToken::new();
    let time = OffsetDateTime::now_utc();

    let statement = db.prepare_cached(CREATE_SESSION).await?;
    db.execute(&statement, &[&id, &game, &team, &token.0, &time])
        .await?;

    db.commit().await?;

    let info = TeamInfo { name };

    Ok(Some((token, info)))
}

pub async fn team_by_session_token(
    db: &mut Client,
    token: SessionToken,
) -> Result<Option<Session>, InternalError> {
    const SESSION_BY_TOKEN: &str = "SELECT game, team FROM session WHERE token=$1";

    let statement = db.prepare_cached(SESSION_BY_TOKEN).await?;
    let row = db.query_opt(&statement, &[&token.0]).await?;

    if let Some(row) = row {
        let game: Uuid = row.try_get(0)?;
        let team: Uuid = row.try_get(1)?;

        Ok(Some(Session { game, team }))
    } else {
        Ok(None)
    }
}

pub async fn team_info(db: &mut Client, game: Uuid, id: Uuid) -> Result<TeamInfo, InternalError> {
    const TEAM_INFO: &str = "SELECT name FROM team WHERE game=$1 AND id=$2";

    let statement = db.prepare_cached(TEAM_INFO).await?;
    let row = db.query_one(&statement, &[&game, &id]).await?;

    let name: String = row.try_get(0)?;

    Ok(TeamInfo { name })
}
