use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_connection_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"))
        .await
        .expect("Error connecting to database")
}
