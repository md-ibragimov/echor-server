use crate::handlers::auth;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
}
