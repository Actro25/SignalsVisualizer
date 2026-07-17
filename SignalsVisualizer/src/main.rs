pub mod handlers;
mod models;
pub mod router;
pub mod generator;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;
use crate::generator::point::Point;

#[tokio::main]
async fn main() {

    let (prod, _): (Sender<Point>, Receiver<Point>) = broadcast::channel(100);
    
    let app = router::router(prod);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "Server is listening on {} .",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
