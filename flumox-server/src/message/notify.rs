use deadpool_postgres::Transaction;
use tokio_postgres::{types::Json, Error};

use super::InvalidateMessage;

pub async fn invalidate(db: &mut Transaction<'_>, message: InvalidateMessage) -> Result<(), Error> {
    const NOTIFY: &str = "SELECT pg_notify('invalidate', cast($1::JSONB AS TEXT))";

    let statement = db.prepare_cached(NOTIFY).await?;
    db.execute(&statement, &[&Json(message)]).await?;

    Ok(())
}
