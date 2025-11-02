use axum::{Router, routing::{get, post, delete}};
use sqlx::{Pool, Postgres};

// Import your attachment handlers
use crate::handlers::attachment_handler::{
    create_attachment,
    get_attachments_by_message,
    delete_attachment,
};

pub fn attachment_routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        // POST /attachments → create a new attachment
        .route("/attachments", post(create_attachment))

        // GET /messages/{message_id}/attachments → get all attachments for a message
        .route("/messages/{message_id}/attachments", get(get_attachments_by_message))

        // DELETE /attachments/{id} → delete an attachment by its id
        .route("/attachments/{id}", delete(delete_attachment))

        // Share the same DB pool with handlers
        .with_state(pool)
}
