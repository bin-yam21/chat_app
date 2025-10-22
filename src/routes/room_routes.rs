use axum::{Router, routing::{get, post}};
use sqlx::{Pool, Postgres};

// Import your handlers
use crate::handlers::{
    room_handler::{get_rooms, create_room},
    message_handler::{get_room_messages, send_message}, // 👈 new imports
};

pub fn room_routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        // Rooms routes
        .route("/rooms", get(get_rooms).post(create_room))

        // 👇 Nested message routes for each room
        // Example: GET  /rooms/{room_id}/messages  → fetch all messages in room
        //          POST /rooms/{room_id}/messages  → send a new message in room
        .route("/rooms/{room_id}/messages", get(get_room_messages).post(send_message))

        // Shared connection pool for all routes
        .with_state(pool)
}
