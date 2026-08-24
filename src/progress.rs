// progress tracking and channel setup.
// mpsc - multi producer and single consumer.
// sender = data send krna and data receive krna.

use tokio::sync::mpsc;


// Ye enum bata hai ki koi task kya report kr raha hai.

#[derive(Debug)]

pub enum ProgressEvent {
    ByteDownloaded(usize, u64),
    ChunkComplete(usize),
    Error(usize,String)
}

// Channel banata hai aur (sender, receiver) return krta hai.
// sender taks ko deta jaye gaa clone krke
// receiver -> main thread rakhega

pub fn create_progress_channel()->(mpsc::Sender<ProgressEvent>,
    mpsc::Receiver<ProgressEvent>){
        // 32 = channel buffer size
        // iska matlab : 32 events queue me rah sakte hai bina block kiye.
        mpsc::channel::<ProgressEvent>(32)
    }


// Progress events receive krta hai aur terminal pr print krata hai 
// ye function main thread par chalega


pub async fn run_progress_report(mut receiver:mpsc::Receiver<ProgressEvent>){
    println!("Progress Report Started... \n");

    // loop chalta rahe jab tk channel band na ho jayee
    // mut receiver kyuki .recv() internally state change krta hai.
    loop {

        match receiver.recv().await {
            Some(ProgressEvent::ByteDownloaded(chunk_id, bytes))=>{
                println!("Chunk {} :{} bytes received", chunk_id, bytes);
            }
            Some(ProgressEvent::ChunkComplete(chunk_id))=>{
                println!("Chunk {} completed",chunk_id)
            }
            Some(ProgressEvent::Error(chunk_id, msg))=>{
                println!("Chunk {} error : {}", chunk_id, msg)
            }
            None =>{
                println!("All task finised!");
                break; // loop se bahar aao
            }
        }
    }
}
