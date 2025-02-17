use std::net::{SocketAddr,};
use axum::response::{Html, IntoResponse};
use axum::{Router, ServiceExt};
use axum::routing::get;


#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_hello.into_make_service()).await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("<h1>Hello</h1>")
}