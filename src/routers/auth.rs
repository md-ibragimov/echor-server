use axum::{routing::post, Router};
use crate::handlers::auth::{login_with_email,login_with_username,signup};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    return Router::new()
        .route("/sign-up", post(signup))
        .route("/email-sign-in", post(login_with_email))
        .route("/username-sign-in", post(login_with_username))
}