use axum::{Router, routing::{get, post}, middleware::from_fn};
use crate::handlers::user_handler::{create_user, get_user, get_users, login};

use crate::auth::require_role;
use sqlx::{Pool, Postgres};

/// Public routes for user-related endpoints (no auth)
pub fn public_routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        // POST /users → register new user (bcrypt + JWT)
        .route("/register", post(create_user))
        // POST /login → login user (bcrypt verify + JWT)
        .route("/login", post(login))
        .with_state(pool)
}

/// Protected routes for user-related endpoints (require auth)
pub fn protected_routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    // Admin-only router for listing users
    let admin_only = Router::new()
        .route("/users", get(get_users))
        .route_layer(from_fn(move |req, next| require_role(req, next, "admin".to_string())));

    // Routes that require auth but have custom checks (e.g., owner or admin)
    let other = Router::new().route("/users/{username}", get(get_user));

    Router::new()
        .merge(admin_only)
        .merge(other)
        .with_state(pool)
}
