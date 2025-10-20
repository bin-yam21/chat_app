use sqlx::{Pool, Postgres};
use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(
        pool: &Pool<Postgres>,
        username: &str,
        email: Option<&str>,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at
            "#
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_username(
        pool: &Pool<Postgres>,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE username = $1
            "#
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_all(
        pool: &Pool<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"SELECT * FROM users ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await
    }
}
