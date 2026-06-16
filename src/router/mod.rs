use axum::{Router, middleware::from_fn, routing::get};
use std::io::Error;

use crate::handlers::auth::verfiy_auth;

fn build_router() -> Router {
    Router::new()
    .layer(from_fn(verfiy_auth))
    .route("/api/v1/health_check", get(|| async { "Working" }))
}

pub async fn start_server() -> Result<(), Error> {
    let app = build_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await
}
