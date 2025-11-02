use sqlx::{Pool, Postgres};
use crate::models::room::Room;
use uuid::Uuid;

pub struct RoomRepository;

impl RoomRepository {
    pub async fn create_room(
        pool: &Pool<Postgres>,
        name: &str,
        created_by: Option<&uuid::Uuid>,
    ) -> Result<Room, sqlx::Error> {
        sqlx::query_as::<_, Room>(
            r#"
            INSERT INTO rooms (name, created_by)
            VALUES ($1, $2)
            RETURNING id, name, created_at, created_by
            "#
        )
        .bind(name)
        .bind(created_by)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all_rooms(pool: &Pool<Postgres>) -> Result<Vec<Room>, sqlx::Error> {
        sqlx::query_as::<_, Room>("SELECT id, name, created_by, created_at FROM rooms ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: &Uuid) -> Result<Option<Room>, sqlx::Error> {
        sqlx::query_as::<_, Room>(
            r#"SELECT id, name, created_by, created_at FROM rooms WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn update_room(
        pool: &Pool<Postgres>,
        id: &Uuid,
        name: &str,
    ) -> Result<Room, sqlx::Error> {
        sqlx::query_as::<_, Room>(
            r#"
            UPDATE rooms SET name = $1 WHERE id = $2
            RETURNING id, name, created_by, created_at
            "#,
        )
        .bind(name)
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_room(pool: &Pool<Postgres>, id: &Uuid) -> Result<bool, sqlx::Error> {
        let res = sqlx::query("DELETE FROM rooms WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected() > 0)
    }
}
