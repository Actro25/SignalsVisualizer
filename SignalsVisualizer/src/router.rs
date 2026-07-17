use crate::generator::point::Point;
use crate::handlers::index::{home, ws_data_transfer_handler};
use axum::{Router, routing::get};
use tokio::sync::broadcast::Sender;
use tower_http::services::ServeDir;

pub fn router(prod: Sender<Point>) -> Router {
    let state = prod;
    Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_data_transfer_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state)
}
