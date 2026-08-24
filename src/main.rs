use cli_downloader::network::{fetch_metadata, calculate_chunks};
use cli_downloader::error::DownloadError;


// #[tokio::main] -> ye macro hai main ko async banata hia.
// eski bina async code nahi chale ga.

#[tokio::main]
async fn main(){
    let url = "https://packaged-media.redd.it/vh77t5m249lh1/pb/m2-res_480p.mp4?m=DASHPlaylist.mpd&var=sgpssan&v=1&e=1787580000&s=f8afa2213ee9bd52cd8f76fa4ef1e0cbe00adcaf";
    println!("Fetching Video details {}",url);
    // fetch metadata async function hai -> .await lagana important hai.
    match fetch_metadata(url).await{
        Ok(metadata)=>{
            println!("File Size : {} bytes ({:.2}) MB)",
            metadata.file_size,
            metadata.file_size as f64/ 1_000_000.0
        );
        println!("Range Supported : {}", metadata.supports_range);
        
        // Testing chunk match test.
        let chunks = calculate_chunks(metadata.file_size, 4);
        println!("\n Chunks (4 threads):");
        for chunk in &chunks{
            println!(" Chunk {}: bytes {} -> {}",chunk.id , chunk.start_byte, chunk.end_byte);            
        }   
    }
    Err(DownloadError::UnknownFileSize)=>{
        eprintln!("File Size not found!")
    }
    Err(DownloadError::NetworkError(e))=>{
        eprintln!("Network Error :{}",e);
    }
    Err(e)=>{
        eprintln!("Error :{}",e);
    }
    }
}