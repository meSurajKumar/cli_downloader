// ─────────────────────────────────────────
//  Tauri invoke helper import
// ─────────────────────────────────────────
const { invoke } = window.__TAURI__.core;
const { WebviewWindow } = window.__TAURI__.webviewWindow;
// ─────────────────────────────────────────
//  DOM Elements
// ─────────────────────────────────────────
const addDownloadBtn = document.getElementById('add-download-btn');


// Overlay click se bhi close ho (box ke bahar click karo)
addDownloadBtn.addEventListener('click', () => {
    const win = new WebviewWindow('add-download', {
        url: 'modal.html',
        title: 'Add New Download',
        width: 520,
        height: 580,
        resizable: false,
        center: true,
        decorations: true,   // OS title bar (close/min/max)
        alwaysOnTop: true,
    });
    win.once('tauri://error', (e) => {
        console.error('Window error:', e);
    });
});
