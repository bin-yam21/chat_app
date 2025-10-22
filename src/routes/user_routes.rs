use axum::{Router, routing::{get, post}};
use crate::handlers::user_handler::{create_user, get_user, get_users, login};

use sqlx::{Pool, Postgres};


pub fn routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        // GET /users → list all users
        // POST /users → register new user (bcrypt + JWT)
        .route("/users", get(get_users).post(create_user))
        .route("/users/{username}", get(get_user))
        // POST /login → login user (bcrypt verify + JWT)
        .route("/login", post(login))
        .with_state(pool)
}
