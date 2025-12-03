use axum::{routing::post, Router};
use crate::handlers::auth::{login_with_email,login_with_username,signup};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    return Router::new()
        .route("/api/sign-up", post(signup))
        .route("/api/email-sign-in", post(login_with_email))
        .route("/api/username-sign-in", post(login_with_username))
}