use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use crate::generator::signals::{Point, Signals};
use crate::handlers::index::{home, ws_data_transfer};
use std::sync::{Arc, Mutex};


pub fn router(cons: impl ringbuf::traits::Consumer<Item = Point> + std::marker::Send + 'static) -> Router{
    let state = Arc::new(Mutex::new(cons));
    Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_data_transfer))
        .fallback_service(ServeDir::new("static"))
        .with_state(state)
}