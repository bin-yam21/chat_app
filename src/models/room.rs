use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Room {
    pub id: i32,                 // SERIAL
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
