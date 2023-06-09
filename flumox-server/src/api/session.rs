use axum::Json;
use serde::{Serialize, Deserialize};

use crate::{
    db, error::InternalError, extract::DbConnection, session::SessionToken, types::TeamInfo,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "result")]
pub enum LoginResponse {
    IncorrectKey,
    Success { token: SessionToken, team: TeamInfo },
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    access_code: String,
}

pub async fn login(
    DbConnection(mut db): DbConnection,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, InternalError> {
    let LoginRequest { access_code: key } = request;

    match db::login(&mut db, &key).await {
        Ok(Some((token, team))) => Ok(Json(LoginResponse::Success { token, team })),
        Ok(None) => Ok(Json(LoginResponse::IncorrectKey)),
        Err(error) => Err(error),
    }
}
