use screencapturekit::content_sharing_picker::*;
use screencapturekit::prelude::*;
use core_graphics2::window::{
    preflight_screen_capture_access,
    request_screen_capture_access,
};
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct CaptureState {
    pub stream: Arc<Mutex<Option<SCStream>>>,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
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
    let app_handle = app.clone();

    SCContentSharingPicker::show(&config, move |outcome| match outcome {
        SCPickerOutcome::Picked(result) => {
            let filter = result.filter();
            
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
        .with_sample_rate(16000);

        let mut stream = SCStream::new(&filter, &config);

        stream.add_output_handler(
            move |sample: CMSampleBuffer, _output_type: SCStreamOutputType| {
                let audio_buffer = sample.audio_buffer_list();

                println!("Received audio buffer: {audio_buffer:?}");
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