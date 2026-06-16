use axum::{Router, routing::get};
use std::io::Error;

fn auth_routes() -> Router {
    Router::new().route("/api/v1/health_check", get(|| async { "Working" }))
}

pub async fn start_server() -> Result<(), Error> {
    let app = Router::new().merge(auth_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await
}
