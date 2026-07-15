pub mod handlers;
mod models;
pub mod router;
pub mod generator;

use askama::Template;
use axum::Router;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{Route, get};
use ringbuf::traits::Split;
use crate::generator::signals;
use crate::generator::signals::Signals;

#[tokio::main]
async fn main() {
    let signal: Signals = Signals::new(100);
    
    let (prod, cons) = signal.ring_buffer.split();
    
    let app = router::router(cons);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "Server is listening on {} .",
        listener.local_addr().unwrap()
    );
    
    tokio::spawn(async move { signals::Signals::generate_data(prod, 0.0, 0.005).await; });

    axum::serve(listener, app).await.unwrap();
}
