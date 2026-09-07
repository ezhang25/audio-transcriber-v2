use futures_util::{
    SinkExt, 
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
};
use tokio::sync::mpsc;

pub fn start_audio_socket() -> mpsc::Sender<Vec<u8>> {
    let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<u8>>(64);

    tauri::async_runtime::spawn(async move {
        let Ok((mut socket, _response)) =
            connect_async("ws://127.0.0.1:8765").await
        else {
            eprintln!("Could not connect audio socket to Python.");
            return;
        };

        println!("Rust audio socket connected to Python.");

        // Tell Python what the following binary messages represent.
        let _ = socket
            .send(Message::Text(
                r#"{"type":"audio-start","sample_rate":48000,"channels":2,"format":"f32le"}"#
                    .into(),
            ))
            .await;

        while let Some(pcm_bytes) = audio_rx.recv().await {
            if let Err(error) = socket.send(Message::Binary(pcm_bytes.into())).await {
                eprintln!("Could not send PCM audio: {error}");
                break;
            }
        }

        let _ = socket.close(None).await;
        println!("Rust audio socket closed.");
    });

    audio_tx
}