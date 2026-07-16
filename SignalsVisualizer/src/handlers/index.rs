use std::sync::{Arc};
use askama::Template;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Response};
use serde::Serialize;
use tokio::sync::Mutex;
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
    loop
    {
        let mut cons_guard = cons.lock().await;
        while let Some(point) = cons_guard.try_pop() {
            let transfer_value: Points = Points{
                x: point.0,
                y: point.1
            };
            match serde_json::to_string(&transfer_value) {
                Ok(json_string) => {
                    if socket.send(Message::Text(json_string.into())).await.is_err() {
                        break;
                    }
                }
                Err(e) => eprintln!("Error JSON serializing: {}", e)
            }
        }
    }
}