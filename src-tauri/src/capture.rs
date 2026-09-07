use screencapturekit::content_sharing_picker::*;
use screencapturekit::prelude::*;
use std::sync::{Arc, Mutex};
use std::process::Child;
use tauri::Manager;

use crate::connection;

pub struct CaptureState {
    pub stream: Arc<Mutex<Option<SCStream>>>,
    pub python_server: Arc<Mutex<Option<Child>>>,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
            python_server: Arc::new(Mutex::new(None)),
        }
    }
}

pub fn select_audio(app: tauri::AppHandle, state: &CaptureState) {
    let mut config = SCContentSharingPickerConfiguration::new();

    config.set_allowed_picker_modes(&[
        SCContentSharingPickerMode::SingleWindow,
        SCContentSharingPickerMode::SingleApplication,
        SCContentSharingPickerMode::SingleDisplay,
    ]);

    let stream_slot = Arc::clone(&state.stream);
    let server_slot = Arc::clone(&state.python_server);
    let app_handle = app.clone();

    SCContentSharingPicker::show(&config, move |outcome| match outcome {
        SCPickerOutcome::Picked(result) => {
            let filter = result.filter();

            let mut active_server = server_slot
                .lock()
                .expect("Could not lock Python server state");

            if let Some(mut old_server) = active_server.take() {
                let _ = old_server.kill();
                let _ = old_server.wait();
            }

            let python_server = std::process::Command::new("../.venv/bin/python")
                .arg("../src-python/main.py")
                .spawn()
                .expect("Python server failed to start");

            *active_server = Some(python_server);
            println!("Python WebSocket server started.");
            
            let source_caption = match result.source() {
                SCPickedSource::Window(title) => format!("Capturing window: {title}"),
                SCPickedSource::Application(name) => format!("Capturing app: {name}"),
                SCPickedSource::Display(id) => format!("Capturing display: {id}"),
                SCPickedSource::Unknown => "Capturing selected source".to_string(),
            };

            if let Some(caption_window) = app_handle.get_webview_window("caption") {
                let text = serde_json::to_string(&source_caption).unwrap();

                let _ = caption_window.eval(&format!(
                    "document.getElementById('caption').textContent = {text};"
                ));
            }

            let capture_slot = Arc::clone(&stream_slot);

            std::thread::spawn(move || {
                match start_audio(filter) {
                    Ok(stream) => {
                        let mut active_stream = capture_slot
                            .lock()
                            .expect("Could not lock capture state");

                        if let Some(old_stream) = active_stream.take() {
                            let _ = old_stream.stop_capture();
                        }

                        *active_stream = Some(stream);
                        println!("Stream started and saved.");
                    }
                    Err(error) => eprintln!("Could not start stream: {error}"),
                }
            });
        }

        SCPickerOutcome::Cancelled => {
            println!("User cancelled picker.");
        }

        SCPickerOutcome::Error(error) => {
            eprintln!("Picker error: {error}");
        }
    });
}

pub fn start_audio(filter: SCContentFilter) -> Result<SCStream, String>  {
    let config = SCStreamConfiguration::new()
        .with_captures_audio(true)
        .with_sample_rate(48_000)
        .with_channel_count(2);

    let audio_tx = connection::start_audio_socket();
    let mut stream = SCStream::new(&filter, &config);

    stream.add_output_handler(
        move |sample: CMSampleBuffer, _output_type: SCStreamOutputType| {
            if let Some(audio_buffer_list) = sample.audio_buffer_list() {
                for buffer in &audio_buffer_list {
                    let pcm_bytes = buffer.data().to_vec();
                    let _ = audio_tx.try_send(pcm_bytes);
                }
            }
        },
        SCStreamOutputType::Audio,
    );
    
    stream
        .start_capture()
        .map_err(|error| error.to_string())?;

    Ok(stream)
}

pub fn end_audio(state: &CaptureState) -> Result<(), String> {
    let mut current_stream = state
        .stream
        .lock()
        .map_err(|_| "Could not lock capture state".to_string())?;

    if let Some(stream) = current_stream.take() {
        stream.stop_capture().map_err(|error| error.to_string())?;
        println!("Audio capture stopped.");
    } else {
        println!("No active audio stream to stop.");
    }

    Ok(())
}

pub fn stop_python_server(state: &CaptureState) {
    let mut server_slot = state
        .python_server
        .lock()
        .expect("Could not lock Python server state");

    if let Some(mut server) = server_slot.take() {
        let _ = server.kill();
        let _ = server.wait();
        println!("Python WebSocket server stopped.");
    }
}