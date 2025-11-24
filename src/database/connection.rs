use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn connect() -> Pool<Postgres> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url is missing in env");

    PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
