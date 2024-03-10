use axum::{routing::get, Router};

pub struct AuthRouter {
    pub router: Router,
}

impl AuthRouter {
    pub fn new() -> Self {
        Self {
            router: Router::new().route("/auth", get(Self::handle_auth())),
        }
    }

    pub fn handle_auth() -> String {
        "Hello, Account".to_string()
    }
}
