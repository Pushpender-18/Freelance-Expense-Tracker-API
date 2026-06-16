use crate::router::start_server;

pub mod router;

#[tokio::main]
async fn main() {
    start_server().await.unwrap_err();
}
