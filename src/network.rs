// network.rs - HTTP se baat karna yahan hoga

// Apne Porject ki types yaha pr import honge
// super:: matlab parent module "lib.rs"

use crate::error::{DownloadError, Result};
use crate::types::{Chunk, FileMetadata};

// reqwest ka async HTTP client
use reqwest::Client;

// Server se file ki metadata fetch krta hai (bina download kiye)
// Return karta hai: FileMetadata struct (size + range support)
pub async fn fetch_metadata(url: &str)-> Result<FileMetadata>{
    // Step 1: Create Http client.
    // Client ek reuable connection pool hai to bar-bar new create nahi krna hoga.

    let client = Client::new();

    let response = client
    .head(url)
    .send()
    .await?;

    let headers = response.headers();

    let file_size = headers.get("content-length").and_then(|v| v.to_str().ok())
    .and_then(|s| s.parse::<u64>().ok())
    .ok_or(DownloadError::UnknownFileSize)?;

    let supports_range = headers.get("accept-range")
    .and_then(|v| v.to_str().ok())
    .map(|v| v=="bytes")
    .unwrap_or(false);

    Ok(FileMetadata { file_size, supports_range, url: url.to_string(),
    })

}

// File ko N chuck me todna hai.

pub fn calculate_chunks(file_size: u64, num_threads: usize)->Vec<Chunk>{
    let chunk_size = file_size / num_threads as u64;
    let mut chunks = Vec::new(); // Empty Vec array  and Vec is the growing array/ dynamic growing array.

    for i in 0..num_threads{
        let id = i;
        let start_byte = i as u64 * chunk_size;
        // Aakhri chunk me remaning byte bhi add ho jayee ge.
        // kyu ki devision exact nahi hote hai 
        let end_byte = if i == num_threads -1 {
            file_size -1 
        }else{
            start_byte + chunk_size -1
        };
        chunks.push(Chunk {id , start_byte , end_byte});   
    
    }
    chunks
}

pub async fn download_chuck(
    client : &Client,
    url : &str,
    chunk : &Chunk
)->Result<Vec<u8>>{

    let range_header = format!("bytes={}-{}",chunk.start_byte, chunk.end_byte);

    let response = client
    .get(url)
    .header(("Range"), range_header)
    .send()
    .await?;

    let status = response.status();
    if !status.is_success(){
        return Err(DownloadError::NetworkError(response.error_for_status().unwrap_err()
    ));
    }
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}


