use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Room {
    pub id: Uuid,                 // SERIAL
    pub name: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
