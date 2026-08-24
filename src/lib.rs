// lib.rs wo file hai jo Rust ko batati hai ki "mere project me ye saare modules hain". Iske bina Rust baaki files ko nahi dekhega.

pub mod types; // Data structure aur Traits
pub mod error; // Custom error types
pub mod cli; // Command-line arguments parsing
pub mod network; // HTTP Reqests
pub mod disk; // File wirting
pub mod progress; // progress bar + channels
