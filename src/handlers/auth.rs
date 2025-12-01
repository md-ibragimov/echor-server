use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, self};
use crate::models::auth::{CreateUserRequest, UserResponse, EmailLoginRequest, UsernameLoginRequest, User};
use serde_json::json;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};


pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, String)> {
    let password_hash = hash(&payload.password, 12)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Password hashing failed: {}", e)))?;

    // Создаем пользователя в БД
    let now = Utc::now();


    let user = sqlx::query_as!(
        User,
        r#"
            INSERT INTO users (username, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        &payload.username,
        &payload.email,
        &password_hash,
        now,
        
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; 

    // Преобразуем в Response
    let user_response = UserResponse {
        id: user.id,
        username: user.username.to_string(),
        email: user.email.to_string(),
    };
    
    return Ok((StatusCode::CREATED, Json(user_response)));
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
