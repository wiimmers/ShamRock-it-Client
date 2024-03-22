const { invoke } = window.__TAURI__.tauri;
const { confirm } = window.__TAURI__.dialog; 
const startButton = document.querySelector('#submitbutton');
window.addEventListener('DOMContentLoaded', async () => {
    startButton.style.display = 'flex';
});