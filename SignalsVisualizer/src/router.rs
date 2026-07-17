use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use crate::handlers::index::{home, ws_data_transfer_handler};
use tokio::sync::broadcast::{Sender};
use crate::generator::point::Point;

pub fn router(prod: Sender<Point>) -> Router{
    let state = prod;
    Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_data_transfer_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state)
}