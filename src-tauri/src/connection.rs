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
        let mut connection = None;

        for attempt in 1..=30 {
            match connect_async("ws://127.0.0.1:8765").await {
                Ok((socket, _response)) => {
                    connection = Some(socket);
                    break;
                }
                Err(error) if attempt < 30 => {
                    println!("Waiting for Python server ({attempt}/30): {error}");
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                Err(error) => {
                    eprintln!("Could not connect audio socket to Python: {error}");
                }
            }
        }

        let Some(mut socket) = connection else {
            return;
        };

        println!("Rust audio socket connected to Python.");

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