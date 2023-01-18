use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/v1/todos/:id", get(get_json));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_json(Path(id): Path<String>) -> impl IntoResponse {
    let message_id = id.as_str();

    (
        StatusCode::OK,
        Json(json!({"id": message_id, "message": "Just do it!",
    "priority": "A",
    "assigned": "user@example.com"})),
    )
}

#[derive(Serialize)]
struct User {
    id: String,
    message: String,
    priority: String,
    assigned: String,
}