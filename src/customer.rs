use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct CustomerNotFoundError {
    pub id: String,
}

impl IntoResponse for CustomerNotFoundError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": format!("Customer with id {} not found", self.id) });
        (StatusCode::NOT_FOUND, Json(body)).into_response()
    }
}

#[derive(Clone)]
pub struct CustomerService {
    customers: Arc<Mutex<HashMap<String, Customer>>>,
}

impl CustomerService {
    pub fn new() -> Self {
        Self {
            customers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_customer(&self, req: CreateCustomerRequest) -> Customer {
        let id = Uuid::new_v4().to_string();
        let customer = Customer {
            id: id.clone(),
            name: req.name,
            email: req.email,
        };

        self.customers
            .lock()
            .unwrap()
            .insert(id.clone(), customer.clone());
        info!("Created customer {:?}", customer);

        customer
    }

    pub fn get_customer(&self, id: String) -> Result<Customer, CustomerNotFoundError> {
        let map = self.customers.lock().unwrap();
        if let Some(c) = map.get(&id) {
            info!("Found customer with id {}", id);
            Ok(c.clone())
        } else {
            info!("Customer not found with id {}", id);
            Err(CustomerNotFoundError { id })
        }
    }
}
