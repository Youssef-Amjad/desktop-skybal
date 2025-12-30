use tauri::{Manager};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                window.set_focus().ok();
            }
        }))
        .run(tauri::generate_context!())
        .expect("error while running skybal application");
}
