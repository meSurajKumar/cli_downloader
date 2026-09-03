const { invoke } = window.__TAURI__.core;
const { WebviewWindow } = window.__TAURI__.webviewWindow;
const { listen } = window.__TAURI__.event;

// ─────────────────────────────────────────
//  DOM Elements
// ─────────────────────────────────────────
const addDownloadBtn = document.getElementById('add-download-btn');
const downloadsList = document.getElementById('downloads-list');
const activeCountEl = document.getElementById('active-count');
const historyTbody = document.getElementById('history-tbody');
const clearHistoryBtn = document.getElementById('clear-history-btn');

// ─────────────────────────────────────────
//  State
// ─────────────────────────────────────────
let activeDownloads = {};  // download_id → { filename, numChunks, ... }

// ─────────────────────────────────────────
//  Add Download Window
// ─────────────────────────────────────────
addDownloadBtn.addEventListener('click', () => {
    const win = new WebviewWindow('add-download', {
        url: 'modal.html',
        title: 'Add New Download',
        width: 520,
        height: 580,
        resizable: false,
        center: true,
        decorations: true,
        alwaysOnTop: true,
    });
    win.once('tauri://error', (e) => console.error('Window error:', e));
});

// ─────────────────────────────────────────
//  Helper: File size format
// ─────────────────────────────────────────
function formatSize(bytes) {
    if (bytes >= 1_000_000_000) return (bytes / 1e9).toFixed(2) + ' GB';
    if (bytes >= 1_000_000) return (bytes / 1e6).toFixed(2) + ' MB';
    if (bytes >= 1_000) return (bytes / 1e3).toFixed(1) + ' KB';
    return bytes + ' B';
}

// ─────────────────────────────────────────
//  Helper: File icon
// ─────────────────────────────────────────
function getFileIcon(type) {
    const icons = {
        Video: '🎬', Audio: '🎵', Image: '🖼',
        Compressed: '📦', Document: '📄', Other: '❓'
    };
    return icons[type] || '📁';
}

function getFileType(filename) {
    const ext = filename.split('.').pop().toLowerCase();
    if (['mp4', 'mkv', 'avi', 'mov', 'webm'].includes(ext)) return 'Video';
    if (['mp3', 'wav', 'flac', 'aac', 'ogg'].includes(ext)) return 'Audio';
    if (['png', 'jpg', 'jpeg', 'gif', 'webp'].includes(ext)) return 'Image';
    if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) return 'Compressed';
    if (['pdf', 'doc', 'docx', 'txt', 'xlsx'].includes(ext)) return 'Document';
    return 'Other';
}

// ─────────────────────────────────────────
//  Download Card banao
// ─────────────────────────────────────────
function createDownloadCard(downloadId, filename, numChunks) {
    const fileType = getFileType(filename);
    const icon = getFileIcon(fileType);

    const chunkRows = Array.from({ length: numChunks }, (_, i) => `
        <div class="chunk-row">
            <span class="chunk-label">Chunk ${i}</span>
            <div class="chunk-bar">
                <div class="chunk-fill" id="cfill-${downloadId}-${i}"></div>
            </div>
            <span class="chunk-pct" id="cpct-${downloadId}-${i}">0%</span>
            <span class="chunk-spd" id="cspd-${downloadId}-${i}">-- MB/s</span>
            <span class="chunk-ico" id="cico-${downloadId}-${i}">⟳</span>
        </div>`).join('');

    return `
    <div class="download-card" id="card-${downloadId}">
        <div class="card-main">
            <div class="file-icon ${fileType.toLowerCase()}">${icon}</div>
            <div class="download-info">
                <div class="filename">${filename}</div>
                <div class="progress-row">
                    <span id="size-${downloadId}">0 MB / ? MB</span>
                    <span id="percent-${downloadId}">0%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" id="fill-${downloadId}"></div>
                </div>
            </div>
            <div class="download-meta">
                <span class="speed" id="speed-${downloadId}">-- MB/s</span>
                <span class="eta"   id="eta-${downloadId}">-- left</span>
            </div>
            <div class="download-actions">
                <button class="details-toggle-btn"
                        id="toggle-btn-${downloadId}"
                        onclick="toggleChunkDetails('${downloadId}')">
                    ⌄ Details
                </button>
                <button id="pause-btn-${downloadId}"
                        onclick="pauseDownload('${downloadId}')">
                    ⏸ Pause
                </button>
                <button class="cancel-btn"
                        onclick="cancelDownload('${downloadId}')">
                    ✕ Cancel
                </button>
            </div>
        </div>
        <div class="chunk-details" id="chunks-${downloadId}" style="display:none">
            <div class="chunk-details-header">Chunk Progress</div>
            ${chunkRows}
        </div>
    </div>`;
}

// ─────────────────────────────────────────
//  Toggle chunk details
// ─────────────────────────────────────────
function toggleChunkDetails(downloadId) {
    const section = document.getElementById(`chunks-${downloadId}`);
    const btn = document.getElementById(`toggle-btn-${downloadId}`);
    const isOpen = section.style.display !== 'none';
    section.style.display = isOpen ? 'none' : 'block';
    btn.textContent = isOpen ? '⌄ Details' : '⌃ Details';
    btn.classList.toggle('active', !isOpen);
}

// ─────────────────────────────────────────
//  Pause / Cancel / Resume
// ─────────────────────────────────────────
async function pauseDownload(downloadId) {
    const btn = document.getElementById(`pause-btn-${downloadId}`);
    const paused = btn.textContent.includes('Pause');
    try {
        if (paused) {
            await invoke('pause_download', { downloadId });
            btn.textContent = '▶ Resume';
        } else {
            await invoke('resume_download', { downloadId });
            btn.textContent = '⏸ Pause';
        }
    } catch (e) { console.error(e); }
}

async function cancelDownload(downloadId) {
    try {
        await invoke('cancel_download', { downloadId });
        document.getElementById(`card-${downloadId}`)?.remove();
        delete activeDownloads[downloadId];
        updateActiveCount();
    } catch (e) { console.error(e); }
}

// ─────────────────────────────────────────
//  Active count update
// ─────────────────────────────────────────
function updateActiveCount() {
    const count = Object.keys(activeDownloads).length;
    activeCountEl.textContent = `(${count})`;
}

// ─────────────────────────────────────────
//  chunk-progress event listen
// ─────────────────────────────────────────
async function setupListeners() {
    await listen('chunk-progress', (event) => {
        const { chunk_id, bytes_downloaded, total_bytes, percent, speed_mbps, status } = event.payload;

        // Download ID nahi milta chunk event mein — sabhi active downloads check karo
        for (const [downloadId, info] of Object.entries(activeDownloads)) {
            const fill = document.getElementById(`cfill-${downloadId}-${chunk_id}`);
            if (!fill) continue;

            document.getElementById(`cfill-${downloadId}-${chunk_id}`).style.width = `${percent.toFixed(0)}%`;
            document.getElementById(`cpct-${downloadId}-${chunk_id}`).textContent = `${percent.toFixed(0)}%`;
            document.getElementById(`cspd-${downloadId}-${chunk_id}`).textContent = `${speed_mbps.toFixed(1)} MB/s`;

            if (status === 'done') {
                document.getElementById(`cico-${downloadId}-${chunk_id}`).textContent = '✅';
                document.getElementById(`cspd-${downloadId}-${chunk_id}`).textContent = '--';
            } else if (status.startsWith('error')) {
                document.getElementById(`cico-${downloadId}-${chunk_id}`).textContent = '❌';
            }

            // Overall progress update
            info.chunkPercents[chunk_id] = percent;
            const avg = Object.values(info.chunkPercents).reduce((a, b) => a + b, 0) / info.numChunks;
            document.getElementById(`fill-${downloadId}`).style.width = `${avg.toFixed(0)}%`;
            document.getElementById(`percent-${downloadId}`).textContent = `${avg.toFixed(0)}%`;
            document.getElementById(`speed-${downloadId}`).textContent = `${speed_mbps.toFixed(1)} MB/s`;
            document.getElementById(`size-${downloadId}`).textContent =
                `${formatSize(bytes_downloaded)} / ${formatSize(total_bytes)}`;
            break;
        }
    });

    // ── Download complete event ──
    await listen('download-complete', (event) => {
        const downloadId = event.payload;
        document.getElementById(`card-${downloadId}`)?.remove();
        delete activeDownloads[downloadId];
        updateActiveCount();
        loadHistory();  // History table refresh karo
    });

    // new-download event — modal se aayega
    await listen('new-download', (event) => {
        const { downloadId, filename, threads } = event.payload;
        window.__addDownload(downloadId, filename, threads);
    });

}

// ─────────────────────────────────────────
//  History Table
// ─────────────────────────────────────────
async function loadHistory() {
    try {
        const entries = await invoke('get_history');
        historyTbody.innerHTML = '';

        // Category counts reset
        const counts = { all: 0, video: 0, audio: 0, image: 0, compressed: 0, document: 0, other: 0 };

        entries.forEach(entry => {
            counts.all++;
            const key = entry.file_type.toLowerCase();
            if (counts[key] !== undefined) counts[key]++;
            else counts.other++;

            historyTbody.innerHTML += `
            <tr>
                <td>${getFileIcon(entry.file_type)} ${entry.filename}</td>
                <td>${formatSize(entry.file_size)}</td>
                <td>${entry.file_type}</td>
                <td>${entry.completed_at}</td>
                <td>${entry.save_path}</td>
                <td>
                    <button onclick="openLocation('${entry.save_path}')">📁</button>
                </td>
            </tr>`;
        });

        // Category counts update
        Object.keys(counts).forEach(key => {
            const el = document.getElementById(`cat-${key}`);
            if (el) el.textContent = counts[key];
        });

    } catch (e) { console.error('History error:', e); }
}

// ─────────────────────────────────────────
//  Open File Location
// ─────────────────────────────────────────
async function openLocation(path) {
    try {
        await invoke('open_file_location', { path });
    } catch (e) { console.error(e); }
}

// ─────────────────────────────────────────
//  Clear History
// ─────────────────────────────────────────
clearHistoryBtn.addEventListener('click', async () => {
    if (!confirm('Sari history delete karein?')) return;
    try {
        await invoke('clear_history');
        loadHistory();
    } catch (e) { console.error(e); }
});

// ─────────────────────────────────────────
//  Download card add karo — modal.js se call hoga
// (window.__addDownload global function)
// ─────────────────────────────────────────
window.__addDownload = function (downloadId, filename, numChunks) {
    activeDownloads[downloadId] = {
        filename,
        numChunks,
        chunkPercents: Array(numChunks).fill(0),
    };
    downloadsList.innerHTML += createDownloadCard(downloadId, filename, numChunks);
    updateActiveCount();
};

// ─────────────────────────────────────────
//  App Init
// ─────────────────────────────────────────
setupListeners();
loadHistory();
