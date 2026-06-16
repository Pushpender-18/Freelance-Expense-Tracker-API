use sqlx::{PgPool, Pool, Postgres};
use std::io::Error;

use crate::router::start_server;

pub mod models;
pub mod repo;
pub mod router;

#[tokio::main]
async fn main() {
    let pool = db_connection().await;
    start_server().await.unwrap_err();
}

async fn db_connection() -> Option<Pool<Postgres>> {
    // TO be moved to .env
    let DB_URL: String = String::from("postgresql://postgres:1234@localhost:5432/expense_api");

    match PgPool::connect(&DB_URL).await {
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
