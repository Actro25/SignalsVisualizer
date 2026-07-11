use axum::Router;
use axum::response::Html;
use axum::routing::{Route, get};

#[tokio::main]
async fn main() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "Server is listening on {} .",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(|| async { Html("Hello World!") }))
}
