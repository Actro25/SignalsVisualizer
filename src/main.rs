pub mod generator;
pub mod handlers;
mod models;
pub mod router;

use crate::generator::point::Point;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

#[tokio::main]
async fn main() {
    let (prod, _): (Sender<Point>, Receiver<Point>) = broadcast::channel(100);

    let app = router::router(prod);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    match listener.local_addr() {
        Ok(addr) => println!("Listening on IP: {}, Port: {}", addr.ip(), addr.port()),
        Err(error) => {
            eprintln!(
                "Failed to get local address. System error kind: {:?}",
                error.kind()
            );
        }
    }

    //Here is using unwrap because Err technically unreachable.
    //If server() has an Error he'll implicitly wrap errors and continue working.
    axum::serve(listener, app).await.unwrap();
}
