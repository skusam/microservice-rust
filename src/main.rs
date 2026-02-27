use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

use microservice_rust::app; // <- kommt aus lib.rs

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
