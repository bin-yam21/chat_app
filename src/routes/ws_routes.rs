use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::handlers::ws_handler::{ws_handler, ChatState};

pub fn ws_routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    // Create broadcast channel (100 message capacity)
    let (tx, _) = tokio::sync::broadcast::channel(100);

    let state = Arc::new(ChatState { pool, tx });

    Router::new()
        .route("/ws/{room_id}", get(ws_handler))
        .with_state(state)
}
