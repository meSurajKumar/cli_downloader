// Modules declare karo — ye files Rust ko batata hai ki
// network.rs, disk.rs etc. is project ka part hain
mod network;
mod disk;
mod types;
mod error;
mod commands; // ← Ye file abhi banayenge
mod state;
mod history;
mod settings;

use state::DownloadRegistry;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(DownloadRegistry::new()))  // ← Global state register karo
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(tauri_plugin_dialog::init())?;
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // ── Purane commands ──
            commands::get_metadata,
            commands::start_download,
            // ── Naye commands (baad mein commands.rs mein banayenge) ──
            commands::cancel_download,
            commands::pause_download,
            commands::resume_download,
            commands::get_file_info,
            commands::get_history,
            commands::clear_history,
            commands::get_settings,
            commands::save_settings_cmd,
            commands::select_folder,
            commands::open_file_location,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}