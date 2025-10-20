use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: i32,     // SERIAL
    pub room_id: i32,
    pub user_id: i32,
    pub content: String,         // not null in your init.sql
    pub created_at: NaiveDateTime,
}
