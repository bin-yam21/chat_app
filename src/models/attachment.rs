use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    pub id: i32,     // SERIAL
    pub message_id: i32,
    pub file_url: String,
    pub file_type: Option<String>,
    pub created_at: NaiveDateTime,
}
