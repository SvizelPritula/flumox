use deadpool_postgres::Client;
use time::OffsetDateTime;
use tokio_postgres::Error;
use uuid::Uuid;

use crate::{
    session::{Session, SessionToken},
    types::{GameInfo, TeamInfo},
};

pub async fn login(db: &mut Client, code: &str) -> Result<Option<(SessionToken, TeamInfo)>, Error> {
    const TEAM_BY_KEY: &str = concat!(
        "SELECT team.game, team.id, team.name, game.name ",
        "FROM team INNER JOIN game ON game.id = team.game ",
        "WHERE team.access_code=$1"
    );
    const CREATE_SESSION: &str =
        "INSERT INTO session (id, game, team, token, created) VALUES ($1, $2, $3, $4, $5)";

    let db = db.transaction().await?;

    let statement = db.prepare_cached(TEAM_BY_KEY).await?;
    let row = db.query_opt(&statement, &[&code]).await?;

    let Some(row) = row else { return Ok(None); };

    let game: Uuid = row.try_get(0)?;
    let team: Uuid = row.try_get(1)?;
    let name: String = row.try_get(2)?;
    let game_name: String = row.try_get(3)?;

    let id = Uuid::new_v4();
    let token = SessionToken::new();
    let time = OffsetDateTime::now_utc();

    let statement = db.prepare_cached(CREATE_SESSION).await?;
    db.execute(&statement, &[&id, &game, &team, &token.0, &time])
        .await?;

    db.commit().await?;

    let info = TeamInfo {
        name,
        game: GameInfo { name: game_name },
    };

    Ok(Some((token, info)))
}

pub async fn team_by_session_token(
    db: &mut Client,
    token: SessionToken,
) -> Result<Option<Session>, Error> {
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

pub async fn team_info(db: &mut Client, game: Uuid, id: Uuid) -> Result<TeamInfo, Error> {
    const TEAM_INFO: &str = concat!(
        "SELECT team.name, game.name ",
        "FROM team INNER JOIN game ON game.id = team.game ",
        "WHERE team.game=$1 AND team.id=$2"
    );

    let statement = db.prepare_cached(TEAM_INFO).await?;
    let row = db.query_one(&statement, &[&game, &id]).await?;

    let name: String = row.try_get(0)?;
    let game_name: String = row.try_get(1)?;

    Ok(TeamInfo {
        name,
        game: GameInfo { name: game_name },
    })
}
