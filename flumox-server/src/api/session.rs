use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    db::{self, team_info, LoginResult},
    error::InternalError,
    extract::{DbConnection, Ip},
    session::{Session, SessionToken},
    types::TeamInfo,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "result")]
pub enum LoginResponse {
    IncorrectCode,
    Success { token: SessionToken, team: TeamInfo },
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    access_code: String,
}

pub async fn login(
    DbConnection(mut db): DbConnection,
    Ip(address): Ip,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, InternalError> {
    let LoginRequest { access_code: key } = request;

    match db::login(&mut db, &key).await {
        Ok(Some(LoginResult {
            game,
            team,
            token,
            info,
        })) => {
            info!(address, %game, %team, "Login succeeded for {name} ({team}) by {address}", name=info.name);
            Ok(Json(LoginResponse::Success { token, team: info }))
        }
        Ok(None) => {
            info!(address, "Login failed, incorrect access key supplied by {address}");
            Ok(Json(LoginResponse::IncorrectCode))
        }
        Err(err) => {
            error!(address, "Failed to verify access code: {err}");
            Err(err.into())
        }
    }
}

pub async fn me(
    Session { game, team }: Session,
    DbConnection(mut db): DbConnection,
) -> Result<Json<TeamInfo>, InternalError> {
    match team_info(&mut db, game, team).await {
        Ok(info) => Ok(Json(info)),
        Err(err) => {
            error!("Failed to obtain team info: {err}");
            Err(err.into())
        }
    }
}
