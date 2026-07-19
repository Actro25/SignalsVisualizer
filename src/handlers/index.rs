use crate::generator::generator::Generator;
use crate::generator::point::Point;
use crate::generator::signals::Signals;
use crate::models::templates::HomeTemplate;
use askama::Template;
use atomic_float::AtomicF64;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Response};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(serde::Deserialize)]
struct Command {
    action: String,
    value: f64,
}
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
        let (sender, receiver) = socket.split();
        let rx_handle = create_new_thread_with_ws_receiver(receiver, amplitude, frequency);
        let tx_handle = create_new_thread_with_ws_sender(sender, consumer);

        let _ = tokio::join!(rx_handle, tx_handle);

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

pub fn create_new_thread_with_ws_receiver(
    mut receiver: SplitStream<WebSocket>,
    amplitude: Arc<AtomicF64>,
    frequency: Arc<AtomicF64>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(message) => match message {
                    Message::Text(msg) => {
                        if let Ok(command) = serde_json::from_str::<Command>(&msg) {
                            match command.action.as_str() {
                                "change_amplitude" => {
                                    amplitude.store(command.value, Ordering::Relaxed);
                                }
                                "change_frequency" => {
                                    frequency.store(command.value, Ordering::Relaxed);
                                }
                                _ => println!("Unseen command."),
                            }
                        }
                    }
                    Message::Close(_) => {
                        println!("Client closed connection");
                        break;
                    }
                    _ => {}
                },
                Err(e) => {
                    println!("WS read error: {e}");
                    break;
                }
            }
        }
    })
}

pub fn create_new_thread_with_ws_sender(
    mut sender: SplitSink<WebSocket, Message>,
    mut cons: Receiver<Point>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match cons.recv().await {
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
    })
}
