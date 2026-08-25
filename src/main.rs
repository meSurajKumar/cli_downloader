use cli_downloader::network::{calculate_chunks, download_chuck, fetch_metadata};
use cli_downloader::error::DownloadError;
use cli_downloader::progress::{create_progress_channel, run_progress_report, ProgressEvent};
use cli_downloader::disk::{wirte_chunk_to_file, create_output_file};

// #[tokio::main] -> ye macro hai main ko async banata hia.
// eski bina async code nahi chale ga.

// use tokio::time::{sleep, Duration};

use reqwest::Client;
use tokio::task::JoinHandle; // Spanwned taks ko track kree ga.

#[tokio::main]
async fn main(){
    // let url = "https://packaged-media.redd.it/vh77t5m249lh1/pb/m2-res_480p.mp4?m=DASHPlaylist.mpd&var=sgpssan&v=1&e=1787580000&s=f8afa2213ee9bd52cd8f76fa4ef1e0cbe00adcaf";
    // let url = "https://speed.hetzner.de/100MB.bin";
    let url = "https://cdn.cocktail.beer/RoboCop.2014.1080p.BluRay.Hindi.English.DD5.1.x264.ESubs.mkv?token=b09d18a51cd56e55b59731a8c08ec899_158";
    let output_path  = "RoboCop.2014.1080p.BluRay.Hindi.English.DD5.1.x264.ESubs.mkv";
    let num_threads = 4;
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
        let chunks = calculate_chunks(metadata.file_size, num_threads);
        println!("Creating output file: {}",output_path);

        if let Err(e) = create_output_file(output_path, metadata.file_size){
            eprintln!("Failed to create file: {}",e);
            return;
        }

        // println!("\n Chunks (4 threads):");
        // println!("\n {} chunk created \n",chunks.len());
        // ---- channel setup -----
        // sender ->task ko denge (clone karke)
        // receiver -> progress report rakhega
        let (sender, receiver) = create_progress_channel();
        let client  = Client::new();
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for chunk in chunks{
            let task_sender = sender.clone();
            let task_client = client.clone();
            let task_url = url.to_string();
            let task_path = output_path.to_string();
            let chunk_id = chunk.id;
            let chunk_start = chunk.start_byte;

            let handle = tokio::spawn(async move {
                println!("Chunk {} Downloading...",chunk_id);
                // Actual Download
                match download_chuck(&task_client, &task_url, &chunk).await {
                    Ok(data)=>{
                        let bytes_count = data.len() as u64;
                        let write_result = tokio::task::spawn_blocking(move || {
                            wirte_chunk_to_file(&task_path, chunk_start, &data)
                        }).await;

                        match write_result  {
                            Ok(Ok(()))=>{
                                task_sender.send(ProgressEvent::ByteDownloaded(chunk_id, bytes_count)).await.ok();
                                task_sender.send(ProgressEvent::ChunkComplete(chunk_id)).await.ok();
                            }
                            Ok(Err(e))=>{
                                task_sender.send(ProgressEvent::Error(chunk_id, e.to_string())).await.ok();
                            }
                            Err(e)=>{
                                task_sender.send(ProgressEvent::Error(chunk_id,e.to_string())).await.ok();
                            }
                        }
                    }
                    Err(e)=>{
                        task_sender.send(ProgressEvent::Error(chunk_id, e.to_string())).await.ok();
                    }
                }
            });
            handles.push(handle);
        }
        // Droping original sender.
        drop(sender);
        run_progress_report(receiver).await;
        for handle in handles{
            handle.await.ok();
        }
        println!("/n Download complete! File saved as: {}",output_path);    
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


    // spaning dummy task for testing
        // for chunk in chunks{
        //     // Har task ki leye ek sender banao
        //     // clone krna padega kyu ki ek sender ek hi task ki pass ho sakta hai.
        //     let task_sender = sender.clone();            
        //     tokio::spawn(async move{
        //         println!("Chunk {}: started bytes {} -> {}",chunk.id , chunk.start_byte, chunk.end_byte);            

        //         // simulatin 3 time fake download
        //         for i in 1..=3{
        //             sleep(Duration::from_millis(500)).await;
        //             // sending event to channel 
        //             // .send().await -> async send
        //             // .ok() -> agar channel band ho to error ignore karoo

        //             task_sender.send(ProgressEvent::ByteDownloaded(chunk.id, i*1000)).await.ok();
        //         }
        //         // task compelete hone pr event send karoo
        //         task_sender.send(ProgressEvent::ChunkComplete(chunk.id)).await.ok();                    
        //     });        
        // }
