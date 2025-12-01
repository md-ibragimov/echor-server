use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::Debug;
use std::clone::Clone;
use validator::{Validate};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: Option<bool>, 
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmailLoginRequest {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UsernameLoginRequest {
    #[validate(length(min = 3, max = 30, message = "Username must be between 3 and 30 characters"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    // pub created_at: DateTime<Utc>,
}