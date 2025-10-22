
use chrono::{DateTime , Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow , Clone)]
pub struct Message {
    pub id: Uuid,     // SERIAL
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub content: Option<String>,         // it can be null when only attachment sent
    pub created_at: DateTime<Utc>,
}
