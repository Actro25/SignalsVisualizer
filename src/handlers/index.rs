use crate::generator::generator::Generator;
use crate::generator::point::Point;
use crate::generator::signals::Signals;
use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use atomic_float::AtomicF64;
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
    let amplitude = Arc::new(AtomicF64::from(1.0));
    let frequency = Arc::new(AtomicF64::from(0.005));

    let signal = Signals::new(working.clone(), amplitude.clone(), frequency.clone());
    create_new_thread_with_signals(signal, prod);

    ws.on_upgrade(async move |socket| {
        send_data_via_ws(socket, consumer, amplitude, frequency).await;
        println!("Web Socket connection was closed.");
        working.store(false, Ordering::Relaxed);
    })
}

fn create_new_thread_with_signals(
    mut signal: impl Generator + Send + 'static,
    prod: Sender<Point>,
) {
    tokio::spawn(async move {
        signal.generate_data(prod).await;
    });
}

async fn send_data_via_ws(socket: WebSocket, mut cons: Receiver<Point>, mut amplitude: Arc<AtomicF64>,mut frequency: Arc<AtomicF64>) {
    let (mut sender, mut receiver) = socket.split();

    loop {
        tokio::select! {
            message = receiver.next() => {
                match message {
                    Some(Ok(Message::Text(msg))) => {
                        println!("Text from Web Socket: {}", msg);
                    },
                    Some(Ok(Message::Close(_))) | None => {
                        println!("Client closed connection");
                        break;
                    }
                    Some(Err(e)) => {
                        println!("WS read error: {e}");
                        break;
                    }
                    _ => {}
                }
            }

            point = cons.recv() => {
                match point {
                    Ok(point) => {
                        let transfer_value: Point = Point {
                            x: point.x,
                            y: point.y,
                        };
                        match serde_json::to_string(&transfer_value) {
                            Ok(json_string) => {
                                if sender
                                    .send(Message::Text(json_string.into()))
                                    .await
                                    .is_err()
                                {
                                    println!("Stopped!!!!!!!!!!!!!!!!!!!!!!!");
                                    break;
                                }
                            }
                            Err(e) => eprintln!("Error JSON serializing: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("There is error in handler_socket: {}", e);
                    }
                }
            }
        }
    }
}
