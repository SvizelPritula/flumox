use std::net::SocketAddr;

use axum::{extract::ConnectInfo, Json};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    db::{self, team_info},
    error::InternalError,
    extract::DbConnection,
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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, InternalError> {
    let LoginRequest { access_code: key } = request;

    match db::login(&mut db, &key).await {
        Ok(Some((token, team))) => {
            info!(%addr, team.name, "Login succeeded");
            Ok(Json(LoginResponse::Success { token, team }))
        }
        Ok(None) => {
            info!(%addr, "Login failed, incorrect access key supplied");
            Ok(Json(LoginResponse::IncorrectCode))
        }
        Err(err) => {
            error!("Failed to verify access code: {err}");
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
