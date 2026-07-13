use std::sync::{Arc, Mutex};
use askama::Template;
use axum::extract::State;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Response};
use serde::Serialize;
use crate::generator::signals::Point;
use crate::models::templates::{HomeTemplate};

pub async fn home() -> Response{
    let html_string = HomeTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


pub async fn ws_data_transfer(ws: WebSocketUpgrade, State(cons): State<Arc<Mutex<impl ringbuf::traits::Consumer<Item = Point> + Send + 'static>>>) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| { handler_socket(socket, cons).await; })
}

#[derive(Serialize)]
pub struct Points{
    x: f64,
    y: f64
}

async fn handler_socket(mut socket: WebSocket, cons: Arc<Mutex<impl ringbuf::traits::Consumer<Item = Point> + Send + 'static>>) {
    let mut counter: u64 = 0;

    let mut x_count: f64 = 5.0;
    let mut y_count: f64 = 10.5;
    loop {
        let transfer_value: Points = Points{
            x: x_count,
            y: y_count
        };

        match serde_json::to_string(&transfer_value) {
            Ok(json_string) => {
                if socket.send(Message::Text(json_string.into())).await.is_err() {
                    break;
                }
            }
            Err(e) => eprintln!("Error JSON serializing: {}", e)
        }

        x_count += 0.1;
        y_count += 0.05;

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}