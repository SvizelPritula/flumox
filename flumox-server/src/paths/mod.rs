use crate::{error::InternalError, extract::DbConnection};

pub async fn time(DbConnection(db): DbConnection) -> Result<String, InternalError> {
    let row = db.query_one("SELECT now() :: text", &[]).await?;
    let result = row.try_get(0)?;
    Ok(result)
}
