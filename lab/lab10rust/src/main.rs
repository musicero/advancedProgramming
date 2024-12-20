use axum::{
    extract,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/greetme", post(greetme))
        .route("/greet/:param_name", get(greet_param));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn hello() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}

#[derive(Serialize, Deserialize)]
struct GreetmeRequest {
    input: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
struct GreetmeResponse {
    greet: String,
    name: String,
}

async fn greetme(
    extract::Json(payload): Json<GreetmeRequest>,
) -> (StatusCode, Json<GreetmeResponse>) {
    let response = GreetmeResponse {
        greet: payload.input,
        name: payload.name,
    };

    (StatusCode::OK, Json(response))
}

#[derive(Serialize, Deserialize)]
struct GreetResponse {
    greet: String,
    name: String,
}
async fn greet_param(Path(param_name): Path<String>) -> (StatusCode, Json<GreetResponse>) {
    let response = GreetResponse {
        greet: String::from("Hello"),
        name: param_name,
    };

    (StatusCode::OK, Json(response))
}
