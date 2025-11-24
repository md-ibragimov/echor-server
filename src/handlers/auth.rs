use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{hash, verify};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateUserRequest, LoginRequest, UserResponse};

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    // Implementation here
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<String>, StatusCode> {
    // Implementation here
    Err(StatusCode::NOT_IMPLEMENTED)
}
