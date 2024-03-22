const { invoke } = window.__TAURI__.tauri; 
const { open } = window.__TAURI__.shell; 
const { message } = window.__TAURI__.dialog; 
const tauriEvent = window.__TAURI__.event;
const ticketForm = document.getElementById('ticket_form'); 
const requestType = document.getElementById('request_type');
const labelRequest = document.querySelector('label[for="request_type"]')
const submitTicket = document.getElementById('submitBtn'); 
const loader = document.getElementById('loader'); 
const sentMsg = document.getElementById('sentMsg');
const ticketType = document.getElementById('ticket_type')
ticketType.addEventListener('change', function () {
    if (ticketType.value == 'Maintenance') {
        requestType.style.display = 'none';
        labelRequest.style.display = 'none'; 
    } else if (ticketType.value == 'IT') {
        requestType.style.display = 'block';
        labelRequest.style.display = 'block'; 
    }
});
ticketType.dispatchEvent(new Event('change'));
// Dropdown menu items
const stores = {
    'Tri County': 
    ["Alfa Romeo", "BMW", "Body Shop","Chevy", "Chrysler", 
     "Jeep", "Mazda", "Mitsubishi", "Office"],
    'Western Hills': 
    ["Mazda"],
    'Florence, KY': 
    ["Body Shop", "Fiat/Kia"],
    'Lebanon, OH': 
    ["GMC"]
};
// Get references to the dropdowns
const locale = document.getElementById('locale');
const dealer = document.getElementById('dealer');
// Add event listener to the first dropdown
locale.addEventListener('change', function () {
    // Get the selected category
    const selectedCategory = locale.value;
    // Clear previous options in the second dropdown
    dealer.innerHTML = '<option value="" selected hidden disabled>Select Store</option>';
    // Populate the second dropdown with items based on the selected category
    stores[selectedCategory].forEach(function (store) {
        const option = document.createElement('option');
        option.value = store;
        option.textContent = store;
        dealer.appendChild(option);
    });
});
// Initialize the second dropdown based on the default selected category
locale.dispatchEvent(new Event('change'));
async function formatMessage () {
    const ticketData = new FormData(ticketForm);
    // Custom object for backend
    const ticketObject = {
        ticket_type: ticketData.get('ticket_type'),
        locale: ticketData.get('locale'),
        dealer: ticketData.get('dealer'),
        first_name: ticketData.get('first_name'),
        last_name: ticketData.get('last_name'),
        email: ticketData.get('email'),
        telephone: ticketData.get('telephone'),
        request_type: ticketData.get('request_type'),
        request: ticketData.get('request'),
    };
    const finalObject = {
        formData: ticketObject,
    };

    if (ticketObject.first_name == "" || ticketObject.last_name == "" || ticketObject.email == "" || ticketObject.request == "") {
        await message('Please fill out required fields', {type: 'warning'}); 
    } else if (ticketObject.ticket_type == 'IT') {
        console.log('Sending IT Ticket');
        submitTicket.style.display = 'none'; 
        ticket_form.style.display = 'none'; 
        loader.style.display = 'block';
        await invoke('ninja_auth', finalObject); 
    } else {
        console.log('Sending maintenance ticket');
        submitTicket.style.display = 'none'; 
        ticket_form.style.display = 'none'; 
        loader.style.display = 'block';
        await invoke('email_ticket', finalObject);
    }
}
tauriEvent.listen('sendingMail', (sendingMail) => {
    console.log('received sendingMail', sendingMail);
    if (sendingMail.payload == 0) {
        submitTicket.style.display = 'none'; 
        loader.style.display = 'block'; 
    } else if (sendingMail.payload == 1) {
        loader.style.display = 'none';
        ticketForm.style.display = 'none'; 
        sentMsg.style.display = 'block'; 
    }
});
tauriEvent.listen('ticketSubmit', async (statusCode) => {
    console.log('received statusCode', statusCode)
    if (statusCode.payload == 200) {
        submitTicket.style.display = 'none';
        loader.style.display = 'none';
        sentMsg.style.display = 'block';
    } else {
        submitTicket.style.display = 'none';
        ticketForm.style.display = 'none';
        await message('ShamRock.it failed to submit the ticket to Ninja One.\nPlease call ext. 3300 for assistance.', {type: 'error'});
    }
})
