use sqlx::{PgPool, Pool, Postgres};
use crate::{router::start_server};

pub mod models;
pub mod repo;
pub mod router;
pub mod handlers;

#[tokio::main]
async fn main() {
    let _pool = db_connection().await;
    start_server().await.unwrap_err();
}

async fn db_connection() -> Option<Pool<Postgres>> {
    // TO be moved to .env
    let db_url: String = String::from("postgresql://postgres:1234@localhost:5432/expense_api");

    match PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("[INFO] Database connection successful");
            Some(pool)
        }
        Err(error) => {
            println!("[ERROR] Database connection failed, {}", error);
            None
        }
    }
}
