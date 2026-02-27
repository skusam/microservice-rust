use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use tower::util::ServiceExt; // for `oneshot`
use serde_json::json;

// Wichtig: Crate-Name aus Cargo.toml → "microservice-rust" → microservice_rust
use microservice_rust::{app, Customer};

#[tokio::test]
async fn test_create_and_get_customer() {
    let app = app();

    // --- 1) POST /customers ---
    let create_body = json!({
        "name": "Alice",
        "email": "alice@example.com"
    });

    let response = app
        .clone()
        .oneshot(
            Request::post("/customers")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let created: Customer = serde_json::from_slice(&bytes).unwrap();

    // --- 2) GET /customers/:id ---
    let response = app
        .clone()
        .oneshot(
            Request::get(format!("/customers/{}", created.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let fetched: Customer = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, "Alice");
}

#[tokio::test]
async fn test_get_unknown_customer() {
    let app = app();

    let response = app
        .oneshot(
            Request::get("/customers/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
