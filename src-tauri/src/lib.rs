mod commands;
mod fs_ops;
mod replace;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::preview,
            commands::apply,
            commands::restore_latest_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
