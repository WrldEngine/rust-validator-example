use sqlx::{Error, Executor, PgPool};

pub async fn create_tables(pool: &PgPool) -> Result<String, Error> {
    let result = pool.execute(
            "CREATE TABLE IF NOT EXISTS validator (
            hash VARCHAR(64) UNIQUE NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_validator_hash ON validator (hash);",
    ).await?;

    Ok(
        format!("Tables Created: {:?}", result)
    )
}