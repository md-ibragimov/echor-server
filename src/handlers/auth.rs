use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;
use crate::models::auth::{CreateUserRequest, UserResponse, EmailLoginRequest, UsernameLoginRequest};


pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    // Implementation here
    Err(StatusCode::NOT_IMPLEMENTED)
    
}

pub async fn login_with_username(
    State(pool): State<PgPool>,
    Json(payload): Json<UsernameLoginRequest>,
) -> Result<Json<String>, StatusCode> {
    // Implementation here
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn login_with_email(
    State(pool): State<PgPool>,
    Json(payload): Json<EmailLoginRequest>,
) -> Result<Json<String>, StatusCode> {
    // Implementation here
    Err(StatusCode::NOT_IMPLEMENTED)
}
