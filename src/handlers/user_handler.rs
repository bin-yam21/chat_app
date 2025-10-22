use axum::{
    extract::{Extension, Json , State , Path},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use std::env;

use crate::repository::UserRepository;

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

// JWT claims structure
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // subject (user id or email)
    exp: usize,  // expiration timestamp
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    match UserRepository::get_all(&pool).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => {
            tracing::error!("failed to get users: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users").into_response()
        }
    }
}

pub  async fn get_user(State(pool): State<Pool<Postgres>>,
 Path(username): Path<String>,
) -> impl IntoResponse {
    match UserRepository::find_by_username(&pool, &username).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("User not found")).into_response(),
        Err(err) => {
            tracing::error!("failed to get the user: {}" , err);
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to fetch the user").into_response()
        }
    }
}

pub async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    // Hash password
    let hashed = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("bcrypt hash error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Password hash failed").into_response();
        }
    };

    match UserRepository::create_user(&pool, &payload.username, payload.email.as_deref(), &hashed).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            tracing::error!("db create user error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response()
        }
    }
}

pub async fn login(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    match UserRepository::find_by_username(&pool,&payload.username).await {
        Ok(Some(user)) => {
            // verify password
            match verify(&payload.password, &user.password_hash) {
                Ok(true) => {
                    // ✅ Generate JWT
                    let secret = match env::var("JWT_SECRET") {
                        Ok(val) => val,
                        Err(_) => {
                            tracing::error!("JWT_SECRET not set in .env");
                            return (StatusCode::INTERNAL_SERVER_ERROR, "Server config error").into_response();
                        }
                    };

                    let expiration = Utc::now()
                        .checked_add_signed(Duration::hours(24))
                        .expect("valid timestamp")
                        .timestamp() as usize;

                    let claims = Claims {
                        sub: user.id.to_string(),
                        exp: expiration,
                    };

                    let token = match encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret.as_ref()),
                    ) {
                        Ok(t) => t,
                        Err(e) => {
                            tracing::error!("JWT encode error: {}", e);
                            return (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed").into_response();
                        }
                    };

                    let resp = AuthResponse { token };
                    (StatusCode::OK, Json(resp)).into_response()
                }
                Ok(false) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
                Err(e) => {
                    tracing::error!("bcrypt verify error: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Auth error").into_response()
                }
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => {
            tracing::error!("db error finding user by email: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Auth error").into_response()
        }
    }
}
