mod customer;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use customer::*;

pub use customer::Customer;

pub fn app() -> Router {
    let service = CustomerService::new();

    Router::new()
        .route("/customers", post(create_customer))
        .route("/customers/{id}", get(get_customer))
        .with_state(service)
}

pub async fn create_customer(
    State(service): State<CustomerService>,
    Json(req): Json<CreateCustomerRequest>,
) -> Json<Customer> {
    Json(service.create_customer(req))
}

pub async fn get_customer(
    State(service): State<CustomerService>,
    Path(id): Path<String>,
) -> Result<Json<Customer>, CustomerNotFoundError> {
    service.get_customer(id).map(Json)
}
