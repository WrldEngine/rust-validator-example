use sqlx::{Error, PgPool, query_as};

#[warn(dead_code)]
pub struct Validator {
    pub hash: Option<String>,
}

pub struct ValidationRepository<'a> {
    pub pool: &'a PgPool,
}

pub trait Repository<'a, T> {
    fn new(pool: &'a PgPool) -> Self;
    async fn add_phrase(&self, hash: &str) -> Result<bool, Error>;
    async fn add_or_exists(&self, hash: &str) -> Result<bool, Error>;
}

impl<'a> Repository<'a, Validator> for ValidationRepository<'a> {
    fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    async fn add_phrase(&self, hash: &str) -> Result<bool, Error> {
        sqlx::query("INSERT INTO validator (hash) VALUES ($1);")
            .bind(hash)
            .execute(self.pool)
            .await?;

        Ok(false)
    }

    async fn add_or_exists(&self, hash: &str) -> Result<bool, Error> {
        let query = query_as!(Validator, "SELECT hash FROM validator WHERE hash = $1 LIMIT 1;", hash)
            .fetch_one(self.pool)
            .await;

        match query {
            Ok(_) => Ok(true),
            Err(Error::RowNotFound) => self.add_phrase(hash).await,
            Err(e) => Err(e),
        }
    }
}
