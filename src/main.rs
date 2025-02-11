use std::net::{SocketAddr, TcpListener};
use axum::http::header::SERVER;
use axum::response::Html;
use axum::{Router, ServiceExt};
use axum::routing::get;
use axum::serve::Serve;


#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("<h1>Hello, World!</h1>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_hello.into_make_service()).await.unwrap();
}
