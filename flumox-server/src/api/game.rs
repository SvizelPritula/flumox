use axum::Json;
use deadpool_postgres::Client;
use flumox::Action;
use serde::Deserialize;
use time::OffsetDateTime;
use tracing::error;
use uuid::Uuid;

use crate::{
    action::{submit_action, SubmissionResponse},
    db::load_state,
    error::InternalError,
    extract::DbConnection,
    session::Session,
    types::WidgetInstance,
    view::render,
};

pub async fn view(
    Session { game, team }: Session,
    DbConnection(db): DbConnection,
) -> Result<Json<Vec<WidgetInstance>>, InternalError> {
    async fn run(
        mut db: Client,
        game: Uuid,
        team: Uuid,
        time: OffsetDateTime,
    ) -> Result<Vec<WidgetInstance>, InternalError> {
        let mut db = db.transaction().await?;
        let (game, meta) = load_state(&mut db, game, team).await?;
        db.commit().await?;

        Ok(render(&game, &meta, time)?)
    }

    let time = OffsetDateTime::now_utc();

    match run(db, game, team, time).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            error!("Failed to render view: {err}");
            Err(err)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Submission {
    widget: Uuid,
    #[serde(flatten)]
    action: Action,
}

pub async fn submit(
    Session { game, team }: Session,
    DbConnection(mut db): DbConnection,
    Json(Submission { widget, action }): Json<Submission>,
) -> Result<Json<SubmissionResponse>, InternalError> {
    match submit_action(&mut db, game, team, widget, action).await {
        Ok(r) => Ok(Json(r)),
        Err(error) => {
            error!("failed to evaluate action: {error}");
            Err(error)
        }
    }
}
