use axum::Router;
use axum::routing::{get, post};
use crate::handlers::index::{home, ws_data_transfer};

pub fn router() -> Router{
    Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_data_transfer))
}