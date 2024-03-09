use axum::Router;

mod middlewares;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().nest("/api", routes::auth::AuthRouter::new().router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}
