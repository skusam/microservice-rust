mod customer;

use axum::{
    extract::{Path, State}, routing::{get, post},
    Json,
    Router,
};
use customer::*;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let service = CustomerService::new();

    let app = Router::new()
        .route("/customers", post(create_customer))
        .route("/customers/:id", get(get_customer))
        .with_state(service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn create_customer(
    State(service): State<CustomerService>,
    Json(req): Json<CreateCustomerRequest>,
) -> Json<Customer> {
    Json(service.create_customer(req))
}

async fn get_customer(
    State(service): State<CustomerService>,
    Path(id): Path<String>,
) -> Result<Json<Customer>, CustomerNotFoundError> {
    service.get_customer(id).map(Json)
}
