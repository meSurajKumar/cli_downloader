use std::collection::HashMap;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;


// -- Ek download ka control handle --
pub struct DownloadControl {
    pub cancel_token: CancellationToken,
    pub pause_tx: watch::Sender<bool>, // true = paused, false = running
}

// -- Global regisrty - saare active download track karta hai --
pub struct DownloadRegistry {
    pub downloads : HashMap<String, DownloadControl>,
    // key = download_id (String UUID)
}

impl DownloadRegistry {
    pub fn new()->Self {
        Self {
            downloads: HashMap::new(),
        }
    }
}