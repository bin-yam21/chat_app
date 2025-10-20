use axum::Router;

pub mod user_routes;


use axum::extract::State;
use sqlx::{Pool, Postgres};



pub fn create_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .nest("/v1",user_routes::routes(pool.clone()))
        .with_state(pool)
}

