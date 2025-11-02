use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::message::Message;

#[derive(Clone)]
pub struct ChatState {
    pub pool: Pool<Postgres>,
    pub tx: broadcast::Sender<(Uuid, Message)>, // room_id + message
}

// Alias for convenience
pub type SharedChatState = Arc<ChatState>;
