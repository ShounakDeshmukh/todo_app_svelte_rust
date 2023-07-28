use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(index));
    let address = SocketAddr::from(([0, 0, 0, 0], 3458));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    format!("Hello world")
}
