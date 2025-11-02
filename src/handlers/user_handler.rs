use axum::{
    extract::{Json, State, Path, Extension},
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
use crate::auth::Claims;

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
struct PublicUser {
    id: String,
    username: String,
    email: Option<String>,
    created_at: String,
    role: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: PublicUser,
}

// We reuse the shared `Claims` type from the auth middleware so the token
// payload and the extension type are the same across middleware and handlers.

#[derive(Serialize)]
struct CreateUserResponse {
    token: String,
    message: String,
    user: PublicUser,
}

pub async fn get_users(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    // Only admin can list all users
    if claims.role != "admin" {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    match UserRepository::get_all(&pool).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => {
            tracing::error!("failed to get users: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users").into_response()
        }
    }
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    match UserRepository::find_by_username(&pool, &username).await {
        Ok(Some(user)) => {
            // Allow access if admin or owner
            if claims.role != "admin" && claims.sub != user.id.to_string() {
                return (StatusCode::FORBIDDEN, "Forbidden").into_response();
            }

            (StatusCode::OK, Json(user)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json("User not found")).into_response(),
        Err(err) => {
            tracing::error!("failed to get the user: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to fetch the user").into_response()
        }
    }
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
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
        Ok(user) => {
            // Generate JWT for the newly created user
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
                role: user.role.clone(),
                exp: expiration,
            };

            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("JWT encode error: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed").into_response();
                }
            };

            let public_user = PublicUser {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                created_at: user.created_at.to_rfc3339(),
                role: user.role,
            };

            let resp = CreateUserResponse {
                token,
                message: "User created".to_string(),
                user: public_user,
            };

            (StatusCode::CREATED, Json(resp)).into_response()
        }
        Err(e) => {
            tracing::error!("db create user error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response()
        }
    }
}

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    match UserRepository::find_by_username(&pool, &payload.username).await {
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
                        role: user.role.clone(),
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

                    // Build public user view (exclude password_hash)
                    let pub_user = PublicUser {
                        id: user.id.to_string(),
                        username: user.username.clone(),
                        email: user.email.clone(),
                        created_at: user.created_at.to_rfc3339(),
                        role: user.role.clone(),
                    };

                    let resp = LoginResponse { token, user: pub_user };
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
