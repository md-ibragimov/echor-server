use axum::{
    extract::{Path, Query, State}, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Json, Router
};
use reqwest;

use serde::{Serialize, Deserialize};
mod database;
mod routers;
mod handlers;
mod models;
use handlers::auth::signup;
use routers::auth::routes;
use database::connection::connect;


#[derive(Serialize)]
struct CreateUser {
    email: String,
    code: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:7870".to_string());

   
    //db connection pull
    let db_connection_pull = connect().await;

    let app = Router::new()
        .route("/", get(hello_world))
        .merge(routes())
        .with_state(db_connection_pull);


    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("server running on {} address", &server_address);
    axum::serve(listener, app).await.unwrap();
}


// async fn get_players(
//     State(pg_connection_pool): State<PgPool>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let rows = sqlx::query_as!(PlayerRow, r#"SELECT * FROM players ORDER BY player_id"#)
//         .fetch_all(&pg_connection_pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({"success": false, "message": e.to_string()}).to_string(),
//             )
//         })?;
//     Ok((
//         StatusCode::OK,
//         json!({"success": true, "data": rows}).to_string(),
//     ))
// }

async fn hello_world() -> &'static str {
    "Hello world"
}
