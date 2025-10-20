use axum::{Router, routing::{get, post}};
use crate::handlers::user_handler::{get_users, create_user, login};

use sqlx::{Pool, Postgres};
use axum::Extension;

pub fn routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        // GET /users → list all users
        // POST /users → register new user (bcrypt + JWT)
        .route("/users", get(get_users).post(create_user))
        // POST /login → login user (bcrypt verify + JWT)
        .route("/login", post(login))
        .layer(Extension(pool))
}
