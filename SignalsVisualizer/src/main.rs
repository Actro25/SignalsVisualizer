pub mod handlers;
mod models;
pub mod router;

use askama::Template;
use axum::Router;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{Route, get};
#[tokio::main]
async fn main() {
    let app = router::router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "Server is listening on {} .",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
