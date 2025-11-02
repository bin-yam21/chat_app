use axum::{middleware::from_fn, Router};
use sqlx::{Pool, Postgres};

// Import your route modules
pub mod user_routes;
pub mod room_routes;
pub mod ws_routes;
pub mod attachment_routes;

use crate::middleware::auth_middleware::require_auth;

pub fn create_routes(pool: Pool<Postgres>) -> Router {
    // Public (no auth) routes — only create_user and login
    let public_routes = Router::new().merge(user_routes::public_routes(pool.clone()));

    // Protected user routes (require auth)
    let protected_user_routes = user_routes::protected_routes(pool.clone()).route_layer(from_fn(require_auth));

    // Protected routes — apply auth middleware to routers that expect JWT
    let protected_rooms = room_routes::room_routes(pool.clone()).route_layer(from_fn(require_auth));
    let protected_attachments = attachment_routes::attachment_routes(pool.clone()).route_layer(from_fn(require_auth));

    // WebSocket router lives on its own state type (Arc<ChatState>) — apply auth middleware as well
    let protected_ws = ws_routes::ws_routes(pool.clone()).route_layer(from_fn(require_auth));

    // Merge public and protected routers
    let v1_routes = Router::new()
        .merge(public_routes)
        .merge(protected_user_routes)
        .merge(protected_rooms)
        .merge(protected_ws)
        .merge(protected_attachments);

    // Nest everything under /api/v1
    Router::new().nest("/api/v1", v1_routes).with_state(pool)
}
