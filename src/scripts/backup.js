const { invoke } = window.__TAURI__.tauri;
const { ask } = window.__TAURI__.dialog; 
const { open } = window.__TAURI__.shell; 
const tauriEvent = window.__TAURI__.event; 
const startButton = document.querySelector('#startbutton');
const progressBar = document.querySelector('#progressbar');
const cancelText = document.querySelector('#cancel'); 
const copyText = document.querySelector('#copy');
const zipText = document.querySelector('#zip');  
const deleteText = document.querySelector('#delete');
const ticketOrBackup = document.querySelector('#ticketorbackup');
const help = document.querySelector('#help');
const legal = document.querySelector('#legal');
let totalSize = 0; 
window.addEventListener('DOMContentLoaded', async () => {
    startButton.style.display = 'flex'; 
    startButton.addEventListener('click', async () => {
        var areYouSure = await ask('ShamRock.it is about to start file compression... \nReady to go?', {type: 'warning'});
        if (areYouSure == true) {
            ticketOrBackup.style.display = 'none';
            help.style.display = 'none';  
            legal.style.display = 'none'; 
            startButton.style.display = 'none';
            cancelText.style.display = 'none';
            progressBar.style.display = 'block'; 
            try {
                await invoke('backup_user');
            } catch (error) {
                console.error('error invoking backup_user', error); 
            }
        } else {
            cancelText.style.display = 'block'; 
        }
    });
    tauriEvent.listen('beginCopy', (beginCopy) => {
        if (beginCopy.payload == 0) {
            zipText.style.display = 'none'; 
            deleteText.style.display = 'none'; 
            copyText.style.display = 'block'; 
        } else {
            copyText.style.display = 'none'; 
        }
    });
    tauriEvent.listen('beginZip', (beginZip) => {
        if (beginZip.payload == 0) {
            zipText.style.display = 'block'; 
            deleteText.style.display = 'none'; 
            copyText.style.display = 'none';
            progressBar.value = 0; 
        } else {
            zipText.style.display = 'none';
        }
    });
    tauriEvent.listen('beginDelete', (beginDelete) => {
        if (beginDelete.payload == 0) {
            zipText.style.display = 'none'; 
            deleteText.style.display = 'block'; 
            copyText.style.display = 'none';
            progressBar.style.display = 'none';
        } else {
            deleteText.innerHTML = 'Compression complete. Zip file located at C:\\tmp';
            ticketOrBackup.style.display = 'block';
            help.style.display = 'block';  
            legal.style.display = 'none';  
            openTemp(); 
        }
    });
    tauriEvent.listen('progressTotal', (total) => {
        totalSize = total.payload; 
    });
    tauriEvent.listen('updateProgress', (position) => {
        try {
            const numericPosition = Number(position.payload); 
            const progressValue = numericPosition / totalSize; 
            
            if (!isNaN(progressValue) && isFinite(progressValue)) {
                progressBar.value += progressValue;
            } else {
                console.error('invalid progress value: ', position); 
            }
        } catch (error) {
            console.error('error updating progress bar', error);
        }
    });
});
async function openTemp() {
    open('C:/tmp')
}
