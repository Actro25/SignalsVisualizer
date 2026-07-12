use askama::Template;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{HomeTemplate};

pub async fn home() -> Response{
    let html_string = HomeTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


pub async fn ws_data_transfer(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handler_socket)
}

async fn handler_socket(mut socket: WebSocket) {
    let mut counter: u64 = 0;
    loop {
        counter += 1;
        let transfer_value = counter.to_string();

        if socket.send(Message::Text(Utf8Bytes::from(transfer_value))).await.is_err() {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}