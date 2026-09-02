use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// App setting struct --
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub default_save_path: String,
    pub default_threads: usize,
    pub theme: String,
    pub max_speed_mbps: f64,
    pub auto_retry: bool,
    pub retry_count: usize,
}

impl Default for AppSettings {
    fn default()->Self {
        Self {
            default_save_path : dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("C:\\Downloads"))
            .to_string_lossy()
            .to_string(),
            default_threads: 4,
            theme : "dark".to_string(),
            max_speed_mbps: 0.0,
            auto_retry: true,
            retry_count: 3
        }
    }
}

// <- Settings file ka path --
fn settings_path()-> PathBuf{
    dirs::data_dir()
    .unwrap_or_else(|| PathBuf::from("."))
    .join("EasyDownloader")
    .join("settings.json")
}

// Settings load karo
pub fn load_setting()-> AppSettings{
    let path = settings_path();
    if !path.exists(){
        return AppSettings::default();
    }
    let data = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

// -- Settings save karo ---
pub fn save_settings(settings: &AppSettings){
    let path = settings_path();
    if let Some(parent) = path.parent(){
        let _ = fs::create_dir_all(parent);
    }
    let data = serde_json::to_string_pretty(settings).unwrap_or_default();
    let _ = fs::write(&path, data);
}