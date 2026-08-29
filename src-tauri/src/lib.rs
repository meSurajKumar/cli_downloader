// Modules declare karo — ye files Rust ko batata hai ki
// network.rs, disk.rs etc. is project ka part hain
mod network;
mod disk;
mod types;
mod error;
mod commands; // ← Ye file abhi banayenge

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        // Commands register karo — JS ye functions call kar sakega
        .invoke_handler(tauri::generate_handler![
            commands::get_metadata,
            commands::start_download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
