// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

mod capture;
mod connection;

#[tauri::command (rename = "startTranscription")]
fn start_transcription(app: tauri::AppHandle, state: tauri::State<capture::CaptureState>) {
    if let Some(existing_window) = app.get_webview_window("caption") {
        let _ = existing_window.show();
        let _ = existing_window.set_focus();
    } else {
        let _caption_window = tauri::WebviewWindowBuilder::new(
            &app,
            "caption",
            tauri::WebviewUrl::App("caption.html".into()),
        )
        .resizable(false)
        .closable(true)
        .always_on_top(true)
        .transparent(true)
        .decorations(false)
        .inner_size(1000.0, 100.0)
        .position(0.0, 800.0)
        .build()
        .expect("Could not create caption window");
    }

    capture::select_audio(app.clone(), state.inner());
}

#[tauri::command (rename = "endTranscription")]
fn end_transcription(app: tauri::AppHandle, state: tauri::State<capture::CaptureState>) {
    if let Some(webview_window) = app.get_webview_window("caption") {
        let _ = webview_window.close();
    }

    match capture::end_audio(state.inner()) {
        Ok(()) => println!("End pressed: audio stream stopped."),
        Err(error) => eprintln!("Could not stop audio stream: {error}"),
    }

    capture::stop_python_server(state.inner());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .manage(capture::CaptureState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_transcription, end_transcription])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            let state = app_handle.state::<capture::CaptureState>();

            let _ = capture::end_audio(state.inner());
            capture::stop_python_server(state.inner());
        }
    });
}
