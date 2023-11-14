use std::error::Error;

use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

type WsTcpStream = WebSocketStream<tokio::net::TcpStream>;

#[derive(Debug, Deserialize)]
struct Request {
    command: String,
    message: String,
}

async fn handle_connection(stream: WsTcpStream) {
    let (mut write, mut read) = stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) if msg.is_text() || msg.is_binary() => {
                let request: Request = serde_json::from_str(msg.to_text().unwrap()).unwrap();

                if request.command == "echo" {
                    // Echo the message back
                    let response = serde_json::json!({
                        "result": request.message,
                    });

                    // Create a text message and send it off.
                    write.send(Message::text(response.to_string())).await.unwrap();
                }
            }
            Ok(_) => (),
            Err(error) => match error {
                WsError::ConnectionClosed | WsError::Protocol(_) | WsError::Utf8 => (),
                err => eprintln!("Error processing message: {}", err),
            },
        }
    }

    println!("Connection closed");
}

async fn start() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind("127.0.0.1:8080").await?;

    // non blocking loop to accept all incoming connnections


    while let Ok((stream, _)) = server.accept().await {
        tokio::spawn(async move {
            let ws_stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Error during the websocket handshake occurred");

            handle_connection(ws_stream).await;
        });
    }
    Ok(())
}