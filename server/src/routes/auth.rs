use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/auth", get(handle_auth))
}

async fn handle_auth() -> &'static str {
    "Hello, User!"
}
