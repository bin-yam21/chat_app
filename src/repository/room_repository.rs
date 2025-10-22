use sqlx::{Pool, Postgres};
use crate::models::room::Room;

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
        sqlx::query_as::<_, Room>("SELECT * FROM rooms ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }
}
