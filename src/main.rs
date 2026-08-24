use cli_downloader::network::{fetch_metadata, calculate_chunks};
use cli_downloader::error::DownloadError;
use cli_downloader::progress::{create_progress_channel, run_progress_report, ProgressEvent};

// #[tokio::main] -> ye macro hai main ko async banata hia.
// eski bina async code nahi chale ga.

use tokio::time::{sleep, Duration};

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
        // println!("\n Chunks (4 threads):");
        println!("\n {} chunk created \n",chunks.len());
        // ---- channel setup -----
        // sender ->task ko denge (clone karke)
        // receiver -> progress report rakhega
        let (sender, receiver) = create_progress_channel();
        
        // spaning dummy task for testing
        for chunk in chunks{
            // Har task ki leye ek sender banao
            // clone krna padega kyu ki ek sender ek hi task ki pass ho sakta hai.
            let task_sender = sender.clone();            
            tokio::spawn(async move{
                println!("Chunk {}: started bytes {} -> {}",chunk.id , chunk.start_byte, chunk.end_byte);            

                // simulatin 3 time fake download
                for i in 1..=3{
                    sleep(Duration::from_millis(500)).await;
                    // sending event to channel 
                    // .send().await -> async send
                    // .ok() -> agar channel band ho to error ignore karoo

                    task_sender.send(ProgressEvent::ByteDownloaded(chunk.id, i*1000)).await.ok();
                }
                // task compelete hone pr event send karoo
                task_sender.send(ProgressEvent::ChunkComplete(chunk.id)).await.ok();                    
            });        
        }
        drop(sender);
        run_progress_report(receiver).await;    
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