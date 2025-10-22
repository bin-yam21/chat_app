use sqlx::{Pool, Postgres};
use crate::models::message::Message;
use uuid::Uuid;

pub struct MessageRepository;

impl MessageRepository {
    pub async fn create_message(
        pool: &Pool<Postgres>,
        room_id: Uuid,
        user_id: Uuid,
        content: Option<&str>,
    ) -> Result<Message, sqlx::Error> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (room_id, user_id, content)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(room_id)
        .bind(user_id)
        .bind(content)
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    pub async fn get_messages_by_room(
        pool: &Pool<Postgres>,
        room_id: Uuid,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let messages = sqlx::query_as::<_, Message>(
            r#"
            SELECT * FROM messages
            WHERE room_id = $1
            ORDER BY created_at ASC
            "#
        )
        .bind(room_id)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }
}
