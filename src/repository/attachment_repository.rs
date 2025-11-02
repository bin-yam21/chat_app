use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::models::attachment::{Attachment, CreateAttachment};

pub struct AttachmentRepository;

impl AttachmentRepository {
    pub async fn create(pool: &Pool<Postgres>, data: CreateAttachment) -> Result<Attachment, sqlx::Error> {
        let record = sqlx::query_as::<_, Attachment>(
            r#"
            INSERT INTO attachments (message_id, file_url, file_type)
            VALUES ($1, $2, $3)
            RETURNING id, message_id, file_url, file_type, created_at
            "#,
        )
        .bind(data.message_id)
        .bind(data.file_url)
        .bind(data.file_type)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    pub async fn get_by_message(pool: &Pool<Postgres>, message_id: Uuid) -> Result<Vec<Attachment>, sqlx::Error> {
        let records = sqlx::query_as::<_, Attachment>(
            r#"
            SELECT * FROM attachments
            WHERE message_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(message_id)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    pub async fn delete(pool: &Pool<Postgres>, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM attachments WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
