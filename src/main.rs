use axum::{
    extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};

use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};
#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  //declare env variable
  let database_url = std::env::var("DATABASE_URL").expect("database_url is missing in env");
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:7870".to_string());

  //db connection pull
  let db_connection_pull = PgPoolOptions::new()
      .max_connections(15)
      .connect(&database_url)
      .await
      .expect("database not connected");

  let app = Router::new()
      .route("/", get(hello_world))
      .route("/players", get(get_players))
      .with_state(db_connection_pull);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("server running on port 3000");
  axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct PlayerRow {
    name: String,
    age: i32,
    wing: i32,
    player_id: i32,
}

async fn get_players(
    State(pg_connection_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
     let rows = sqlx::query_as!(PlayerRow, r#"SELECT * FROM players ORDER BY player_id"#)   
         .fetch_all(&pg_connection_pool)
         .await
         .map_err(|e| {
             (
                 StatusCode::INTERNAL_SERVER_ERROR,
                 json!({"success": false, "message": e.to_string()}).to_string(),
             )
         })?;
     Ok((
         StatusCode::OK,
         json!({"success": true, "data": rows}).to_string(),
     ))
}

async fn hello_world() -> &'static str {
    "Hello world"
}
