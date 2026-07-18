use crate::generator::point::Point;
use crate::generator::signaler::Signaler;
use crate::generator::signals::Signals;
use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Response};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast::{Receiver, Sender};

pub async fn home() -> Response {
    let html_string = HomeTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn ws_data_transfer_handler(
    ws: WebSocketUpgrade,
    State(prod): State<Sender<Point>>,
) -> impl IntoResponse {
    let consumer = prod.subscribe();
    let working = Arc::new(AtomicBool::new(true));
    let wr1 = working.clone();
    let wr2 = working.clone();

    let signal = Signals::new(wr1);
    create_new_thread_with_signals(signal, prod);

    ws.on_upgrade(async move |socket| {
        send_data_via_ws(socket, consumer).await;
        wr2.store(false, Ordering::Relaxed);
    })
}

fn create_new_thread_with_signals(mut signal: impl Signaler + Send + 'static, prod: Sender<Point>) {
    tokio::spawn(async move {
        signal.generate_data(prod).await;
    });
}

async fn send_data_via_ws(mut socket: WebSocket, mut cons: Receiver<Point>) {
    loop {
        match cons.recv().await {
            Ok(point) => {
                let transfer_value: Point = Point {
                    x: point.x,
                    y: point.y,
                };
                match serde_json::to_string(&transfer_value) {
                    Ok(json_string) => {
                        if socket
                            .send(Message::Text(json_string.into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(e) => eprintln!("Error JSON serializing: {}", e),
                }
            }
            Err(e) => {
                println!("There is exception in handler_socket: {}", e);
            }
        }
    }
}
