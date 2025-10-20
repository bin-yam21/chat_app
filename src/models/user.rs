use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,                // ✅ matches UUID in Postgres
    pub username: String,
    pub email: Option<String>,   // ✅ matches `email TEXT UNIQUE` (nullable by default)
    pub password_hash: String,
    pub created_at: DateTime<Utc>
}
