const { invoke } = window.__TAURI__.core;
const { getCurrent } = window.__TAURI__.webviewWindow;
const { writeText, readText } = window.__TAURI__.clipboardManager;

// DOM Elements
const modalUrl = document.getElementById('modal-url');
const customFilename = document.getElementById('custom-filename');
const outputDir = document.getElementById('output-dir');
const pasteBtn = document.getElementById('paste-btn');
const browseBtn = document.getElementById('browse-btn');
const refreshBtn = document.getElementById('refresh-details-btn');
const cancelBtn = document.getElementById('modal-cancel');
const downloadBtn = document.getElementById('modal-download');

// Current window reference
const thisWindow = getCurrent();

// ── Cancel → window band karo ──
cancelBtn.addEventListener('click', () => thisWindow.close());

// ── Paste ──
pasteBtn.addEventListener('click', async () => {
    const text = await readText();
    modalUrl.value = text;
});

// ── Browse Folder ──
browseBtn.addEventListener('click', async () => {
    const selected = await invoke('select_folder');
    if (selected) outputDir.value = selected;
});

// ── Refresh Details ──
refreshBtn.addEventListener('click', async () => {
    const url = modalUrl.value.trim();
    if (!url) return;

    refreshBtn.textContent = '⟳ Loading...';
    refreshBtn.disabled = true;

    try {
        const info = await invoke('get_file_info', { url });
        document.getElementById('detail-size').textContent = info.size || '--';
        document.getElementById('detail-type').textContent = info.content_type || '--';
        document.getElementById('detail-ranges').textContent = info.accepts_range ? 'Yes ✅' : 'No ❌';
        document.getElementById('detail-date').textContent = info.last_modified || '--';
    } catch (e) {
        alert('Error: ' + e);
    } finally {
        refreshBtn.textContent = '↺ Refresh Details';
        refreshBtn.disabled = false;
    }
});

// ── Download Button ──
downloadBtn.addEventListener('click', async () => {
    const url = modalUrl.value.trim();
    if (!url) { alert('URL daalo!'); return; }

    const outputPath = outputDir.value;
    const filename = customFilename.value.trim() || null;

    downloadBtn.textContent = '⟳ Starting...';
    downloadBtn.disabled = true;

    try {
        const downloadId = await invoke('start_download', {
            url,
            filename,
            outputPath,
            threads: 4,
        });

        // Download start ho gaya — window close karo
        thisWindow.close();

    } catch (e) {
        alert('Error: ' + e);
        downloadBtn.textContent = '⬇ Download';
        downloadBtn.disabled = false;
    }
});
