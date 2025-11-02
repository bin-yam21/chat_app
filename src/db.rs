use sqlx::{Pool, Postgres};
use dotenvy::dotenv;
use std::{env, time::Duration};
use tokio::time::sleep;

pub type Dbpool = Pool<Postgres>;

pub async fn init_db() -> Dbpool {
    dotenv().ok(); // Load .env

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut retries = 10; // number of retries
    loop {
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Connected to Postgres successfully!");
                break pool;
            }
            Err(e) => {
                if retries == 0 {
                    panic!("Failed to create pool after retries: {:?}", e);
                }
                retries -= 1;
                println!("Postgres not ready yet, retrying in 5s... ({})", retries);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
