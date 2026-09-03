const { invoke } = window.__TAURI__.core;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;
// const { writeText, readText } = window.__TAURI__.clipboardManager;

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
const thisWindow = getCurrentWebviewWindow();


// ── Cancel → window band karo ──
cancelBtn.addEventListener('click', () => thisWindow.close());

// ── Fetch Details helper ──
async function fetchDetails(url) {
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
        console.error('File info error:', e);
    } finally {
        refreshBtn.textContent = '↺ Refresh Details';
        refreshBtn.disabled = false;
    }
}



// ── Paste ──
pasteBtn.addEventListener('click', async () => {
    const text = await navigator.clipboard.readText();
    modalUrl.value = text;
    fetchDetails(text);
});

// ── Browse Folder ──
browseBtn.addEventListener('click', async () => {
    const selected = await invoke('select_folder');
    if (selected) outputDir.value = selected;
});

// ── Refresh Details ──
// refreshBtn.addEventListener('click', async () => {
//     const url = modalUrl.value.trim();
//     if (!url) return;

//     refreshBtn.textContent = '⟳ Loading...';
//     refreshBtn.disabled = true;

//     try {
//         const info = await invoke('get_file_info', { url });
//         document.getElementById('detail-size').textContent = info.size || '--';
//         document.getElementById('detail-type').textContent = info.content_type || '--';
//         document.getElementById('detail-ranges').textContent = info.accepts_range ? 'Yes ✅' : 'No ❌';
//         document.getElementById('detail-date').textContent = info.last_modified || '--';
//     } catch (e) {
//         alert('Error: ' + e);
//     } finally {
//         refreshBtn.textContent = '↺ Refresh Details';
//         refreshBtn.disabled = false;
//     }
// });
// ── Refresh Details ──
refreshBtn.addEventListener('click', () => {
    fetchDetails(modalUrl.value.trim());
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
        // Download start ho gaya — main window ko batao
        const mainWin = await window.__TAURI__.webviewWindow.WebviewWindow.getByLabel('main');
        if (mainWin) {
            await mainWin.emit('new-download', {
                downloadId,
                filename: customFilename.value.trim() || url.split('/').pop().split('?')[0],
                threads: 4,
            });
        }
        thisWindow.close();


    } catch (e) {
        alert('Error: ' + e);
        downloadBtn.textContent = '⬇ Download';
        downloadBtn.disabled = false;
    }
});
