use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// -- History entry struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryEntry {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub file_size: u64,
    pub save_path: String,
    pub file_type: String,
    pub completed_at: String
}

pub fn get_file_category(filename: &str)-> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str(){
                "mp4" | "mkv" | "avi" | "mov" | "webm"    => "Video",
        "mp3" | "wav" | "flac" | "aac" | "ogg"    => "Audio",
        "png" | "jpg" | "jpeg" | "gif" | "webp"   => "Image",
        "zip" | "rar" | "7z" | "tar" | "gz"       => "Compressed",
        "pdf" | "doc" | "docx" | "txt" | "xlsx"   => "Document",
        _                                           => "Other",
    }
}

// -- History file ka path --
fn history_path()->PathBuf{
    // Appdata/Roaming/EasyDownloader/history.json
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("EasyDownloader")
        .join("history.json")
}


//  History load karo file se
pub fn load_history() -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let path = history_path();

    if !path.exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(&path)?;
    let history = serde_json::from_str(&data)?;

    Ok(history)
}

// History save karo file mein
pub fn save_history(entries: &Vec<HistoryEntry>){
    let path = history_path();
    // parent dir banao agar exist nahi karti
    if let Some(parent) = path.parent(){
        let _ = fs::create_dir_all(parent);
    }
    let data = serde_json::to_string_pretty(entries).unwrap_or_default();
    let _ = fs::write(&path, data);
}

// -- Naya entry add karo ---
pub fn add_history_entry(entry: HistoryEntry){
    let mut entries = load_history();
    entries.insert(0,entry); // latest phale
    save_history(&entries)
}
