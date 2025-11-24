use axum::{
    extract::{Path, Query, State}, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Json, Router
};
use reqwest;

use serde::{Serialize, Deserialize};
mod database;

#[derive(Serialize)]
struct CreateUser {
    email: String,
    code: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:7870".to_string());

     match send_post().await {
         Ok(response) => {
             println!("Status: {}", response.status());
             let body = response.text().await.unwrap();
             println!("Body: {}", body);
         }
         Err(e) => {
             eprintln!("Error: {}", e);
         }
    }
    
    //db connection pull
    let db_connection_pull = database::connection::connect().await;

    let app = Router::new()
        .route("/", get(hello_world))
        .with_state(db_connection_pull);

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("server running on {} address", &server_address);
    axum::serve(listener, app).await.unwrap();
}


async fn send_post() -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let user = CreateUser {
        email: "raisky061001@yandex.ru".to_string(),
        code: "1234".to_string(),
    };
    let res = client.post("http://localhost:3000/send-email")
        .json(&user)
        .send()
        .await?;
    return Ok(res);
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
