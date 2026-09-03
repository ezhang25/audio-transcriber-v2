// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command (rename = "startTranscription")]
async fn start_transcription(app: tauri::AppHandle) {
    let start_server = std::process::Command::new("python3")
        .arg("../src-python/main.py")
        .spawn()
        .expect("python server failed to start");

    let webview_window = tauri::WebviewWindowBuilder::new(&app, "caption", tauri::WebviewUrl::App("caption.html".into()))
        .resizable(false)
        .closable(true)
        .always_on_top(true)
        .transparent(true)
        .decorations(false)
        .inner_size(1000.0, 100.0)
        .position(0.0, 0.0)
        .build()
        .unwrap();
}

#[tauri::command (rename = "endTranscription")]
fn end_transcription(app: tauri::AppHandle) {
    if let Some(webview_window) = app.get_webview_window("caption") {
        let _ = webview_window.close();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_transcription, end_transcription])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
