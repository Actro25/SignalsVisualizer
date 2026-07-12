use axum::routing::{get, Router};
use tower_http::services::ServeDir;
use crate::handlers::index::{home, ws_data_transfer};

pub fn router() -> Router{
    Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_data_transfer))
        .fallback_service(ServeDir::new("static"))
}