const { invoke } = window.__TAURI__.tauri;
const { ask } = window.__TAURI__.dialog; 
const tauriEvent = window.__TAURI__.event; 
const fullPath = document.querySelector('#fullpath');
const folderString = document.querySelector('#folders');
const userMessage = document.querySelector('#user'); 
const readmeContainer = document.getElementById('readme');
let fullPathString;
let folderFullString;
let isFetchingUser = false; 
window.addEventListener('DOMContentLoaded', async () => {
    console.log('content loaded'); 
    try {
        if(!isFetchingUser) {
            isFetchingUser = true;
            await invoke('get_user_frontend');
            console.log('get_user invoked');
        }
    } catch (error) {
        console.error('Error invoking get_user_frontend', error); 
    } finally {
        isFetchingUser = false; 
    }
}); 
tauriEvent.listen('currentUser', (user) => {
    console.log('received currentUser from backend:', user); 
    let userString = user.payload; 
    fullPathString = `Full path: C:\\tmp\\${userString}.zip`;
    folderFullString = `The path to these folders is C:\\Users\\${userString}\\`
    fullPath.innerHTML = fullPathString;
    folderString.innerHTML = folderFullString;
    userMessage.innerHTML = userString;
    showReadmeContent(); 
});
function showReadmeContent() {
    readmeContainer.style.display = 'block'; 
}


