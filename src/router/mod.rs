use axum::{Router, middleware::from_fn, routing::{get, post}};
use std::io::Error;

use crate::handlers::{auth::verfiy_auth, expense::expense_add};

fn build_router() -> Router {
    Router::new()
        .route("/api/v1/expense/add", post(expense_add))
        .layer(from_fn(verfiy_auth))    // Middleware
        .route("/api/v1/health_check", get(|| async { "Working" })) // Health Check
}

pub async fn start_server() -> Result<(), Error> {
    let app = build_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await
}
