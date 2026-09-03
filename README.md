# ⬇️ EasyDownloader — Multi-threaded Desktop Download Manager

A blazing-fast, parallel file downloader built with **Rust + Tauri** — featuring a modern dark UI, real-time progress tracking, pause/cancel support, download history, and file categorization.

> 🚧 **Status:** Active Development — Core engine + GUI complete, Settings panel in progress.

---

## ✨ Features

- ⚡ **Parallel Chunk Downloading** — Splits files into N chunks, downloads each simultaneously
- 🌐 **HTTP Range Requests** — Uses `Range: bytes=start-end` headers for partial downloads
- 🔍 **File Info Probe** — HEAD request se file size, content type, range support detect karta hai
- 📡 **Async I/O** — Tokio runtime pe built — non-blocking, high performance
- 💾 **Safe Disk Writing** — Pre-allocated file + seeked writes — no corruption
- 📊 **Real-time Progress** — Live per-chunk progress bars with speed (MB/s)
- ⏸️ **Pause / Resume** — CancellationToken + watch channel se download control
- ✕ **Cancel** — Instant cancellation with registry cleanup
- 🕐 **Download History** — JSON file mein persist hoti hai, app restart ke baad bhi
- 📂 **File Categories** — Auto-categorize: Video, Audio, Image, Compressed, Document
- 📁 **Open File Location** — Windows Explorer mein directly open karo
- 🌙 **Dark Theme UI** — Modern glassmorphism-inspired design
- 🗂️ **Add Download Window** — Separate OS window with native title bar

---

## 🏗️ Architecture

```
EasyDownloader
├── Frontend (HTML/CSS/JS)          ← Tauri Webview
│   ├── index.html                  ← Main window (2-column layout)
│   ├── style.css                   ← Dark theme, grid layout, animations
│   ├── main.js                     ← App logic, event listeners, history
│   ├── modal.html                  ← Add Download window (separate OS window)
│   └── modal.js                    ← Modal logic, folder picker, download trigger
│
└── Backend (Rust / src-tauri)
    ├── main.rs                     ← Tauri entry point
    ├── lib.rs                      ← Module declarations, global state
    ├── commands.rs                 ← All Tauri commands (invoke handlers)
    ├── state.rs                    ← DownloadRegistry (pause/cancel control)
    ├── history.rs                  ← HistoryEntry, JSON persistence
    ├── settings.rs                 ← AppSettings, JSON persistence
    ├── network.rs                  ← HTTP HEAD probe, Range GET, chunk download
    ├── disk.rs                     ← File pre-allocation, seeked writes
    ├── types.rs                    ← Chunk, DownloadConfig, FileMetadata
    └── error.rs                    ← Custom DownloadError enum
```

---

## 🔄 How It Works

```
User clicks "+ Add Download"
        │
        ▼
New OS Window opens (modal.html)
        │
        ├── URL input karo
        ├── "Refresh Details" → HEAD request → file size, type, ranges
        ├── "Browse" → Folder picker (tauri-plugin-dialog)
        │
        ▼
"Download" button click
        │
        ├── invoke('start_download') → Rust backend
        │       ├── UUID generate (download_id)
        │       ├── fetch_metadata() → file size
        │       ├── calculate_chunks() → byte ranges
        │       ├── create_output_file() → pre-allocate disk
        │       ├── CancellationToken + pause channel create
        │       ├── DownloadRegistry mein register
        │       └── tokio::spawn() → parallel chunk downloads
        │               ├── Range GET request
        │               ├── Progress emit → "chunk-progress" event
        │               ├── seeked write to disk
        │               └── All done → add_history_entry() + "download-complete" event
        │
        ▼
Main window updates:
        ├── Download card show (progress bar, speed, pause/cancel)
        ├── On complete → card remove, history table refresh
        └── Category counts update
```

---

## 🚀 Quick Start

### Prerequisites

- **Rust** (stable) — [rustup.rs](https://rustup.rs)
- **Node.js** — [nodejs.org](https://nodejs.org)
- **Tauri CLI** — `cargo install tauri-cli`
- **WebView2** (Windows) — usually pre-installed

### Run in Development

```bash
# Clone the repo
git clone https://github.com/meSurajKumar/cli_downloader.git
cd cli_downloader

# Dev server start karo
cargo tauri dev
```

### Build for Production

```bash
cargo tauri build
```

---

## 🧰 Tech Stack

### Backend (Rust)

| Crate | Version | Use |
|-------|---------|-----|
| `tauri` | 2.x | Desktop app framework |
| `tokio` | 1.x | Async runtime |
| `reqwest` | 0.12 | HTTP client (HEAD + Range GET) |
| `tokio-util` | 0.7 | `CancellationToken` for cancel/pause |
| `uuid` | 1.x | Unique download IDs |
| `serde` / `serde_json` | 1.x | JSON serialization (history, settings) |
| `chrono` | 0.4 | Timestamps for history entries |
| `dirs` | 5.x | Platform-agnostic AppData paths |
| `thiserror` | 2.x | Custom error types |
| `tauri-plugin-dialog` | 2.x | Native folder picker |
| `tauri-plugin-log` | 2.x | Debug logging |

### Frontend

| Technology | Use |
|-----------|-----|
| HTML5 | Structure (semantic layout) |
| Vanilla CSS | Dark theme, CSS Grid, animations |
| Vanilla JS | App logic, event handling |
| Tauri JS API | `invoke`, `listen`, `WebviewWindow` |

---

## 📁 Project Structure

```
cli_downloader/
├── frontend/
│   ├── index.html          ← Main window
│   ├── style.css           ← Complete UI styling
│   ├── main.js             ← Main window logic
│   ├── modal.html          ← Add Download window
│   └── modal.js            ← Modal logic
│
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json    ← Tauri permissions
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands.rs     ← All invoke commands
│       ├── state.rs        ← Global download registry
│       ├── history.rs      ← Download history
│       ├── settings.rs     ← App settings
│       ├── network.rs      ← HTTP layer
│       ├── disk.rs         ← File I/O
│       ├── types.rs        ← Shared types
│       └── error.rs        ← Error handling
│
└── README.md
```

---

## 📋 Tauri Commands

| Command | Description |
|---------|-------------|
| `start_download` | Download start karo, `download_id` return karo |
| `cancel_download` | Download cancel karo |
| `pause_download` | Download pause karo |
| `resume_download` | Paused download resume karo |
| `get_file_info` | URL ka HEAD request — size, type, ranges |
| `get_history` | Sari history load karo |
| `clear_history` | History delete karo |
| `get_settings` | App settings load karo |
| `save_settings_cmd` | Settings save karo |
| `select_folder` | Native folder picker open karo |
| `open_file_location` | Windows Explorer mein file open karo |
| `get_metadata` | File metadata fetch karo |

---

## 🗺️ Roadmap

### ✅ Done

- [x] Parallel chunk downloading (Tokio async tasks)
- [x] HTTP Range requests
- [x] Pre-allocated seeked file writes
- [x] Tauri GUI — dark theme, 2-column layout
- [x] Real-time chunk progress bars
- [x] Add Download modal (separate OS window)
- [x] Pause / Resume / Cancel downloads
- [x] Download history (JSON persistence)
- [x] File auto-categorization
- [x] Open file location (Windows Explorer)
- [x] Native folder picker
- [x] Global download registry (state management)

### 🔜 In Progress

- [ ] Settings panel (default path, threads, speed limit)
- [ ] ETA calculation
- [ ] Speed limiting
- [ ] Retry logic (N retries per failed chunk)
- [ ] Light theme toggle
- [ ] Drag & drop URL support

---

## 🧠 Key Rust Concepts Used

| Concept | Kahan Use Hua |
|---------|---------------|
| `async/await` | Network requests, Tauri commands |
| `tokio::spawn` | Parallel chunk download tasks |
| `Arc<AtomicUsize>` | Thread-safe chunk completion counter |
| `CancellationToken` | Download cancel support |
| `watch::channel` | Pause/resume signal broadcast |
| `Mutex<T>` + `State<T>` | Global download registry in Tauri |
| `serde` | JSON history/settings persistence |
| `SeekFrom::Start` | Writing at specific file offsets |
| `Result<T, E>` + `?` | Error propagation without panics |

---

## 📝 License

MIT License

---

## 🙋 Author

Built as a deep-dive project to master **Rust async**, **systems programming**, and **Tauri desktop development**.
