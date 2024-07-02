use dotenv::dotenv;
use log::error;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool() -> PgPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    
    let result = PgPoolOptions::new()
        .max_connections(100000)
        .connect(&database_url)
        .await;

    match result {
        Ok(pool) => {
            if pool.is_closed() {
                error!("create_pool: database connection is closed!");
                panic!();
            }
            pool
        }
        Err(e) => {
            error!("create_pool: Error connecting to the database: {}", e);
            panic!();
        }
    }
}