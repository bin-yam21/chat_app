use sqlx::{Pool, Postgres};
use dotenvy::dotenv;
use std::env;

pub type Dbpool = Pool<Postgres>;

pub async fn init_db() -> Dbpool {
    dotenv().ok(); //loads env 

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool.")
}