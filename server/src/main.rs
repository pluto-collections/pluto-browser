use std::env;

use axum::Router;
use diesel::{Connection, Insertable, PgConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::user::User;

mod middlewares;
mod models;
mod routes;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    // Example
    let user = User {
        id: 2,
        username: "admin".to_string(),
        full_name: "Admin".to_string(),
        email: "random".to_string(),
        password: "password".to_string(),
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    User::add_user(user);

    let routes_all = Router::new().nest("/api", routes::auth::AuthRouter::new().router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}
