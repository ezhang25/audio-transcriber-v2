use futures_util::SinkExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
};

pub fn connect_to_python() {
    tauri::async_runtime::spawn(async {
        let connection = connect_async("ws://127.0.0.1:8765").await;

        println!("Sucessfully connected to Python websocket!");

        match connection {
            Ok((mut stream, reponse)) => {
                if let Err(error) = stream
                    .send(Message::Text("Hello from Rust!".into()))
                    .await
                {
                    eprintln!("Could not send WebSocket message: {error}");
                }
            }
            Err(error) => {
                eprintln!("Could not connect to Python WebSocket: {error}");
            }
        }
    });
}