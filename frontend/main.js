const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// DOM elements
const urlInput = document.getElementById('url-input');
const threadsInput = document.getElementById('threads');
const infoBtn = document.getElementById('info-btn');
const downloadBtn = document.getElementById('download-btn');
const fileInfo = document.getElementById('file-info');
const filenameText = document.getElementById('filename-text');
const filesizeText = document.getElementById('filesize-text');
const progressSection = document.getElementById('progress-section');
const progressFill = document.getElementById('progress-fill');
const percentText = document.getElementById('percent-text');
const statusText = document.getElementById('status-text');
const overallSpeed = document.getElementById('overall-speed');
const chunksContainer = document.getElementById('chunks-container');

let currentMetadata = null;

// Chunk progress track karne ke liye
// { chunk_id: { percent, speed } }
let chunkProgress = {};

// ── Chunk Cards Dynamically Banao ──
function createChunkCards(numChunks) {
    chunksContainer.innerHTML = ''; // Pehle clear karo
    chunkProgress = {};

    for (let i = 0; i < numChunks; i++) {
        chunkProgress[i] = { percent: 0, speed: 0 };

        chunksContainer.innerHTML += `
        <div class="chunk-card" id="chunk-card-${i}">
            <div class="chunk-header">
                <span class="chunk-label">Chunk ${i}</span>
                <div class="chunk-meta">
                    <span class="chunk-speed" id="chunk-speed-${i}">0 MB/s</span>
                    <span class="chunk-status" id="chunk-status-${i}">⟳</span>
                </div>
            </div>
            <div class="chunk-bar">
                <div class="chunk-fill" id="chunk-fill-${i}"></div>
            </div>
            <div class="chunk-percent" id="chunk-percent-${i}">0%</div>
        </div>`;
    }
}

// ── Overall Progress Calculate karo (average of all chunks) ──
function updateOverallProgress() {
    const ids = Object.keys(chunkProgress);
    if (ids.length === 0) return;

    // Average percent
    const avgPercent = ids.reduce((sum, id) => sum + chunkProgress[id].percent, 0) / ids.length;

    // Total speed (sum of all chunks)
    const totalSpeed = ids.reduce((sum, id) => sum + chunkProgress[id].speed, 0);

    progressFill.style.width = `${avgPercent.toFixed(0)}%`;
    percentText.textContent = `${avgPercent.toFixed(0)}%`;
    overallSpeed.textContent = `${totalSpeed.toFixed(1)} MB/s`;

    // Sab done?
    if (avgPercent >= 100) {
        statusText.textContent = '✅ Download Complete!';
        overallSpeed.textContent = '—';
        downloadBtn.disabled = false;
    }
}

// ── GET INFO BUTTON ──
infoBtn.addEventListener('click', async () => {
    const url = urlInput.value.trim();
    if (!url) { alert('URL daalo pehle!'); return; }

    infoBtn.textContent = 'Loading...';
    infoBtn.disabled = true;

    try {
        const meta = await invoke('get_metadata', { url });
        currentMetadata = meta;

        filenameText.innerHTML = `Filename: <span>${meta.filename}</span>`;
        filesizeText.innerHTML = `Size: <span>${meta.file_size_mb.toFixed(2)} MB</span>`;
        fileInfo.style.display = 'block';

    } catch (error) {
        alert('Error: ' + error);
    } finally {
        infoBtn.textContent = 'Get Info';
        infoBtn.disabled = false;
    }
});

// ── DOWNLOAD BUTTON ──
downloadBtn.addEventListener('click', async () => {
    if (!currentMetadata) return;

    const url = urlInput.value.trim();
    const threads = parseInt(threadsInput.value) || 4;
    const outputPath = currentMetadata.filename;

    // UI setup
    progressSection.style.display = 'block';
    progressFill.style.width = '0%';
    percentText.textContent = '0%';
    statusText.textContent = 'Downloading...';
    overallSpeed.textContent = '0 MB/s';
    downloadBtn.disabled = true;

    // Chunk cards banao
    createChunkCards(threads);

    // chunk-progress events listen karo
    const unlisten = await listen('chunk-progress', (event) => {
        const { chunk_id, percent, speed_mbps, status } = event.payload;

        // Chunk progress store karo
        chunkProgress[chunk_id] = { percent, speed: speed_mbps };

        // Chunk card update karo
        const fill = document.getElementById(`chunk-fill-${chunk_id}`);
        const speedEl = document.getElementById(`chunk-speed-${chunk_id}`);
        const pctEl = document.getElementById(`chunk-percent-${chunk_id}`);
        const statEl = document.getElementById(`chunk-status-${chunk_id}`);

        if (fill) fill.style.width = `${percent.toFixed(0)}%`;
        if (pctEl) pctEl.textContent = `${percent.toFixed(0)}%`;
        if (speedEl) speedEl.textContent = `${speed_mbps.toFixed(1)} MB/s`;

        if (status === 'done') {
            if (statEl) statEl.textContent = '✅';
            if (speedEl) speedEl.textContent = '—';
            chunkProgress[chunk_id].speed = 0;
        } else if (status.startsWith('error')) {
            if (statEl) statEl.textContent = '❌';
        }

        // Overall progress update karo
        updateOverallProgress();
    });

    try {
        await invoke('start_download', { url, threads, outputPath });
    } catch (error) {
        statusText.textContent = '❌ Error: ' + error;
        downloadBtn.disabled = false;
        unlisten();
    }
});
