use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};

pub async fn connect() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url is missing in env");

    PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
