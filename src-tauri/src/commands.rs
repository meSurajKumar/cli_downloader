use tauri::{Window, Emitter};
use serde::Serialize;
use std::time::Instant;
use crate::network::{fetch_metadata, calculate_chunks, download_chunk_with_progress};
use crate::disk::{create_output_file, wirte_chunk_to_file};
use crate::state::{DownloadRegistry};
use crate::history::{HistoryEntry, load_history};
use crate::settings::{AppSettings, load_setting, save_settings};
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tauri::State;
use std::sync::Mutex;
use uuid::Uuid;


//-- file size info ki leye struct -
#[derive(Serialize, Clone)]
pub struct FileInfo{
    pub size: String,
    pub content_type: String,
    pub accepts_range: bool,
    pub last_modified: String,
}

// Js ko bhejea jaayega -> Serialize zaroori
#[derive(Serialize, Clone)]
pub struct MetadataResult {
    pub file_size: u64,
    pub file_size_mb: f64,
    pub supports_range: bool,
    pub filename: String,
}
#[derive(Serialize, Clone)]
pub struct ChunkProgressPayload {
    pub chunk_id: usize,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub percent: f64,
    pub speed_mbps: f64,
    pub status: String,
}



// Get File info model refresh details
#[tauri::command]
pub async fn get_file_info(url: String)->Result<FileInfo, String>{
    let client = reqwest::Client::new();
    let resp = client.head(&url).send().await.map_err(|e| e.to_string())?;

    let headers = resp.headers();

    let size = headers.get("content-length").and_then(|v| v.to_str().ok()).and_then(|s| s.parse::<u64>().ok())
    .map(|b| format!("{:.2} MB", b as f64 / 1_000_000.0))
    .unwrap_or("Unknown".to_string());

    let content_type = headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("Unknown").to_string();

    let accepts_range = headers.get("accept-ranges").and_then(|v| v.to_str().ok()).map(|v| v == "bytes").unwrap_or(false);

    let last_modified= headers.get("last-modified").and_then(|v| v.to_str().ok()).unwrap_or("Unknown").to_string();
    Ok(FileInfo {size, content_type, accepts_range, last_modified})

} 

// cancle download
#[tauri::command]
pub fn cancel_download(
    download_id: String,
    registry: State<Mutex<DownloadRegistry>>,
)->Result<(), String>{
    let mut reg = registry.lock().unwrap();
    if let  Some(ctrl) = reg.downloads.remove(&download_id){
        ctrl.cancel_token.cancel();
        Ok(())
    }else{
        Err("Download not found".to_string())
    }
}

// Pause Download
#[tauri::command]
pub fn pause_download(
    download_id: String,
    registry: State<Mutex<DownloadRegistry>>,
)-> Result<(), String>{
    let reg = registry.lock().unwrap();
    if let Some(ctrl) = reg.downloads.get(&download_id){
        ctrl.pause_tx.send(true).map_err(|e| e.to_string())?;
        Ok(())
    }else{
        Err("Download not found".to_string())
    }
}

//  Resume download
#[tauri::command]
pub fn resume_download(
    download_id: String,
    registry: State<Mutex<DownloadRegistry>>,
)-> Result<(), String>{
    let reg = registry.lock().unwrap();
    if let Some(ctrl) = reg.downloads.get(&download_id){
        ctrl.pause_tx.send(false).map_err(|e| e.to_string())?;
        Ok(())
    }else{
        Err("Download not found".to_string())
    }
}

//  Get history
#[tauri::command]
pub fn get_history()->Vec<HistoryEntry>{
    load_history()
}

// Clear History
#[tauri::command] 
pub fn clear_history()->Result<(), String>{
    crate::history::save_history(&vec![]);
    Ok(())
}

//  Get Settings
#[tauri::command]
pub fn get_settings()->AppSettings{
    load_setting()
}

// ─────────────────────────────────
//  save_settings
// ─────────────────────────────────
#[tauri::command]
pub fn save_settings_cmd(settings: AppSettings) -> Result<(), String> {
    save_settings(&settings);
    Ok(())
}
// ─────────────────────────────────
//  select_folder — folder picker dialog
// ─────────────────────────────────
#[tauri::command]
pub async fn select_folder(app: tauri::AppHandle) -> Option<String> {
    use tauri_plugin_dialog::DialogExt;
    use tokio::sync::oneshot;
    let (tx, rx) = oneshot::channel();

    app.dialog()
        .file()
        .pick_folder(move |folder| {
            let _ = tx.send(folder);
        });
    rx.await.ok().flatten().map(|p| p.to_string())
}
// ─────────────────────────────────
//  open_file_location — Explorer mein open karo
// ─────────────────────────────────
#[tauri::command]
pub fn open_file_location(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .args(["/select,", &path])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}



// JS call: invoke('get_metadata', { url })
#[tauri::command]
pub async fn get_metadata(url: String)-> Result<MetadataResult, String>{
    match fetch_metadata(&url).await{
        Ok(meta)=> Ok(MetadataResult {
            file_size: meta.file_size,
            file_size_mb: meta.file_size as f64 / 1_000_000.0,
            supports_range: meta.supports_range,
            filename: url.split('/').last().unwrap_or("file").split("?").next().unwrap_or("file").to_string(), 
        }),
        Err(e)=> Err(e.to_string()),
    }
}

// JS call: invoke('start_download', { url, threads, outputPath })
#[tauri::command]
pub async fn start_download(url: String, threads: usize, output_path: String, window: Window)-> Result<String, String>{
    let metadata = fetch_metadata(&url).await.map_err(|e| e.to_string())?;
    let chunks = calculate_chunks(metadata.file_size, threads);
    create_output_file(&output_path, metadata.file_size).map_err(|e| e.to_string())?;
    let client = reqwest::Client::new();
    for chunk in chunks{
        let window_clone = window.clone();
        let client_clone = client.clone();
        let url_clone = url.clone();
        let path_clone = output_path.clone();
        let total = metadata.file_size;
        let chunk_size = chunk.end_byte - chunk.start_byte + 1;

        tokio::spawn(async move {
            let start_time = Instant::now(); // Speed measure krne ki leye
            let result = download_chunk_with_progress(
                &client_clone,&url_clone,&chunk,
                |bytes_so_far|{
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 {
                        bytes_so_far as f64 / elapsed / 1_000_000.0
                    }else {0.0};
                    let percent = (bytes_so_far as f64 / chunk_size as f64) *100.0;
                    // har peice pr js ko update bhejo
                    window_clone.emit("chunk-progress", ChunkProgressPayload{
                        chunk_id: chunk.id,
                        bytes_downloaded: bytes_so_far,
                        total_bytes: chunk_size,
                        percent: percent.min(100.0),
                        speed_mbps: speed,
                        status: "downloading".to_string(),
                    }).ok();
                }
            ).await;
            match result {
                Ok(data) => {
                    wirte_chunk_to_file(&path_clone, chunk.start_byte, &data).ok();

                    // Compeltion event
                    window_clone.emit("chunk-progress", ChunkProgressPayload{
                        chunk_id: chunk.id,
                        bytes_downloaded: chunk_size,
                        total_bytes: chunk_size,
                        percent: 100.0,
                        speed_mbps:0.0,
                        status: "done".to_string(),
                    }).ok();
                }
                Err(e) =>{
                    window_clone.emit("chunk-progress", ChunkProgressPayload{
                        chunk_id: chunk.id,
                        bytes_downloaded: 0,
                        total_bytes: chunk_size,
                        percent: 0.0,
                        speed_mbps:0.0,
                        status: format!("error: {}",e),
                    }).ok();
                }
            }
        });
    }Ok("Download started!".to_string())
}