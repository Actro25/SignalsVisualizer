use axum::Router;
use axum::routing::get;
use crate::handlers::index::home;

pub fn router() -> Router{
    Router::new().route("/", get(home))
}