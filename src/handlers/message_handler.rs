use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::repository::message_repository::MessageRepository;
use crate::models::message::Message;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub user_id: Uuid,
    pub content: Option<String>,
}

// POST /rooms/{room_id}/messages
pub async fn send_message(
    State(pool): State<Pool<Postgres>>,
    Path(room_id): Path<Uuid>,
    Json(body): Json<CreateMessageRequest>,
) -> Result<Json<Message>, (axum::http::StatusCode, String)> {
    let result = MessageRepository::create_message(
        &pool,
        room_id,
        body.user_id,
        body.content.as_deref(),
    )
    .await;

    match result {
        Ok(message) => Ok(Json(message)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// GET /rooms/{room_id}/messages
pub async fn get_room_messages(
    State(pool): State<Pool<Postgres>>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<Vec<Message>>, (axum::http::StatusCode, String)> {
    let result = MessageRepository::get_messages_by_room(&pool, room_id).await;

    match result {
        Ok(messages) => Ok(Json(messages)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
