# ⬇️ RustDL — Multi-threaded CLI File Downloader

A blazing-fast, parallel file downloader built from scratch in **Rust** — designed to maximize download speeds by splitting files into chunks and downloading them concurrently using async I/O.

> 🚧 **Status:** Active Development — Core engine complete, GUI integration planned.

---

## ✨ Features

- ⚡ **Parallel Chunk Downloading** — Splits files into N chunks, downloads each simultaneously
- 🌐 **HTTP Range Requests** — Uses `Range: bytes=start-end` headers for partial downloads
- 🔍 **Server Probing** — HEAD request se file size aur range support detect karta hai
- 📡 **Async I/O** — Tokio runtime pe built — non-blocking, high performance
- 💾 **Safe Disk Writing** — Pre-allocated file + seeked writes — no corruption
- 📊 **Real-time Progress** — MPSC channels se live download events track karta hai
- 🛡️ **Custom Error Handling** — `thiserror` se type-safe errors, no panics

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                        main.rs                          │
│          (Tokio Runtime + Orchestration)                │
└──────┬──────────┬──────────┬──────────┬────────────────┘
       │          │          │          │
       ▼          ▼          ▼          ▼
  network.rs   disk.rs  progress.rs  error.rs
  (HTTP)       (I/O)    (Channels)   (Types)
       │          │          │
       └──────────┴──────────┘
                  │
              types.rs
         (Shared Data Structs)
```

### Module Responsibilities

| Module | Kaam |
|--------|------|
| `main.rs` | Tokio runtime init, orchestration, task spawning |
| `network.rs` | HTTP HEAD probe, Range GET requests, chunk download |
| `disk.rs` | Pre-allocate file, seeked concurrent writes |
| `progress.rs` | MPSC channel setup, progress event handling |
| `error.rs` | Custom `DownloadError` enum, `Result<T>` alias |
| `types.rs` | `Chunk`, `DownloadConfig`, `FileMetadata`, `ChunkStatus` |

---

## 🔄 How It Works

```
1. HEAD Request  →  File size + Range support detect karo
2. Chunk Math    →  File ko N equal parts me divide karo
3. File Alloc    →  Disk pe full-size empty file banao
4. Spawn Tasks   →  Har chunk ke liye ek Tokio async task
5. Range GET     →  Har task apna specific byte range download kare
6. Seeked Write  →  Apne offset pe directly file me likho
7. Progress      →  MPSC channel se main thread ko updates bhejo
```

---

## 🚀 Quick Start

### Prerequisites

- Rust (stable) — [rustup.rs](https://rustup.rs)

### Build & Run

```bash
# Clone the repo
git clone <your-repo-url>
cd cli_downloader

# Build
cargo build

# Run
cargo run
```

### Example Output

```
🔍 Probing server...
✅ Size: 104.86 MB | Range: true
📁 Creating output file: output.bin

🚀 Chunk 0 downloading...
🚀 Chunk 1 downloading...
🚀 Chunk 2 downloading...
🚀 Chunk 3 downloading...

Chunk 0: 26214400 bytes received
Chunk 0 completed
...
✅ Download complete! File saved as: output.bin
```

---

## 🧰 Tech Stack

| Crate | Version | Use |
|-------|---------|-----|
| `tokio` | 1.x | Async runtime (`#[tokio::main]`, `spawn`, `spawn_blocking`) |
| `reqwest` | 0.12 | HTTP client (HEAD + GET with Range headers) |
| `thiserror` | 2.x | Custom error types with `#[from]` auto-conversion |
| `futures` | 0.3 | Async utilities |
| `indicatif` | 0.17 | Progress bar (planned) |
| `clap` | 4.x | CLI argument parsing (planned) |

---

## 📁 Project Structure

```
cli_downloader/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs         # Entry point, Tokio runtime
    ├── lib.rs          # Module declarations
    ├── types.rs        # Core data structures & enums
    ├── error.rs        # Custom DownloadError enum
    ├── network.rs      # HTTP requests, Range headers
    ├── disk.rs         # File I/O, seeked concurrent writes
    └── progress.rs     # MPSC channels, progress tracking
```

---

## 🗺️ Roadmap

### ✅ Done
- [x] Project architecture & module setup
- [x] Custom error handling with `thiserror`
- [x] Server metadata probe (HEAD request)
- [x] Chunk calculation (byte range math)
- [x] Parallel async task spawning (Tokio)
- [x] MPSC channel-based progress tracking
- [x] Range GET download per chunk
- [x] Pre-allocated seeked file writes

### 🔜 Planned
- [ ] CLI arguments (`--url`, `--threads`, `--output`) via `clap`
- [ ] Real-time progress bar with speed (MB/s) + ETA via `indicatif`
- [ ] Range fallback (single-stream for non-range servers)
- [ ] Retry logic (3 retries per failed chunk)
- [ ] Resume support (download jahan ruka wahan se)
- [ ] Checksum verification (MD5/SHA256)
- [ ] **Floem GUI** — Native desktop UI with live progress bars

---

## 🧠 Key Rust Concepts Used

| Concept | Kahan Use Hua |
|---------|---------------|
| `async/await` | Network requests, task coordination |
| `tokio::spawn` | Parallel chunk download tasks |
| `tokio::task::spawn_blocking` | File I/O (blocking) in async context |
| `mpsc::channel` | Cross-task progress communication |
| `Result<T, E>` + `?` operator | Error propagation without panics |
| `#[from]` in `thiserror` | Auto error type conversion |
| `SeekFrom::Start` | Writing at specific file offsets |
| `Clone` on `Client` | Cheap shared HTTP connection pool |

---

## 📝 License

MIT License

---

## 🙋 Author

Built as a learning project to master Rust's async ecosystem — Tokio, ownership, and systems programming.
