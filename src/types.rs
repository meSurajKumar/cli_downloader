// types.rs pure project ke blueprint yaha pr hai.


//  Ek download chunk ko represent krta hai.
// jab tum file ko 8 ports me todenge ge, to har port me ek chuck ho ga.

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: usize, /// chunk number (0, 1, 2...)
    pub start_byte: u64, // is chunk ka phela byte
    pub end_byte: u64 // is chunk ka ending byte
}

// User ka download configuration
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub url: String, // Download Url
    pub output_path: String, // File kahan save hoge
    pub threads: usize // kitne parallel threads

}

// Ek chuck ki current state
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkStatus {
    Pending,
    Downloading,
    Done,
    Failed,
    Pause
}

#[derive(Debug, Clone)]
pub struct FileMetadata{
    pub file_size: u64,
    pub supports_range: bool,
    pub url: String
}