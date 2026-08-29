// error.rs pure project ki errors yahan pr define hai.
// thiserror crate bohot saare boilerplate khud likh deta hai.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    // reqwest se jo bhi error aaye, automatically DownloadError ban jata hai
    // {0} matlab error ka message

    #[error("Disk error : {0}")]
    DiskError(#[from] std::io::Error),
    // File read/write ki leye

    #[error("Server Does not support range requests")]
    RangeNotSupported,
    // Jab server multi-part download support na kare

    #[error("Invalid Url: {0}")]
    InvalidUrl(String),
    // Jab Url Galat Ho

    #[error("File size unknown")]
    UnknownFileSize
    // Jab server file size na bataye
}

pub type Result<T> = std::result::Result<T, DownloadError>;