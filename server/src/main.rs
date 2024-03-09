use axum::Router;

mod middlewares;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().nest("/api", routes::auth::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}
