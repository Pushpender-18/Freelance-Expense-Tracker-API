use crate::{repo::expenses::delete_expense, router::start_server};
use sqlx::{PgPool, Pool, Postgres};

pub mod handlers;
pub mod models;
pub mod repo;
pub mod router;

#[tokio::main]
async fn main() {
    let _pool = db_connection().await.unwrap(); // Fix unwrap
    delete_expense(2, &_pool).await.unwrap();
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
