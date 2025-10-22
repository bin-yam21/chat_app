use axum::Router;
use sqlx::{Pool, Postgres};

// Import your route modules
pub mod user_routes;
pub mod room_routes;
pub mod ws_routes;

pub fn create_routes(pool: Pool<Postgres>) -> Router {
    // Merge all v1 routes together
    let v1_routes = Router::new()
        .merge(user_routes::routes(pool.clone()))
        .merge(room_routes::room_routes(pool.clone()))
        .merge(ws_routes::ws_routes(pool.clone()));
        

    // Nest everything under /v1
    Router::new()
        .nest("/v1", v1_routes)
        .with_state(pool)
}
