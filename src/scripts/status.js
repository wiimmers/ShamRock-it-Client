const { invoke } = window.__TAURI__.tauri; 
const tauriEvent = window.__TAURI__.event; 

const interval = setInterval(invokeTicketStatuses, 1000)
invoke('ticket_statuses')
async function invokeTicketStatuses() {
	invoke('ticket_statuses')
}

tauriEvent.listen('ticketObject', async (ticketObject) => {
    let jsonData = JSON.parse(ticketObject.payload)
    await insertList(jsonData)
});

async function insertList(jsonData) {
	var listDiv = document.getElementById('list');
	listDiv.innerHTML = ''; 
	for (var i = 0; i < jsonData.length; i++) {
		var newParagraph = document.createElement('p'); 
		newParagraph.style.display = 'flex'; 
		newParagraph.style.flexDirection = 'row';
		newParagraph.style.alignItems = 'center'; 
		newParagraph.style.borderBottom = 'solid gray';


		if (jsonData[i].status === 'NEW') {
			newParagraph.style.color = 'red'
		} else if (jsonData[i].status === 'OPEN') {
			newParagraph.style.color = 'orange'
		} else if (jsonData[i].status === 'WAITING') {
			newParagraph.style.color = 'blue'
		} else if (jsonData[i].status === 'PAUSED') {
			newParagraph.style.color = 'gray'
		} else if (jsonData[i].status === 'RESOLVED') {
			newParagraph.style.color = 'green'
		}


		var statusDiv = document.createElement('div')
		statusDiv.style.position = 'relative'
		var ticketImg = document.createElement('img'); 
		ticketImg.src = 'assets/ticket.png';
		ticketImg.style.width = '110px';

		statusDiv.appendChild(ticketImg)

		newParagraph.appendChild(statusDiv);

		for(var key in jsonData[i]) {
			var text = document.createElement('span');

			switch (key) {
				case 'status': 
					text.style.fontSize = '18px'
					text.style.position = 'absolute'
					text.style.textAlign = 'center'
					text.style.top = '60px'
					text.style.left = '5px'
					text.style.fontStyle = 'italic'
					text.style.fontWeight = 'bolder'
				case 'ticketNo':
					text.style.width = '100px';
					break;
			}
            if (key === 'status') {
				text.textContent = jsonData[i][key]
				statusDiv.appendChild(text)
				if (text.textContent === 'RESOLVED' || text.textContent === 'WAITING' || text.textContent === 'PAUSED') {
					text.style.fontSize = '13px'
				}
			}
			else {
				text.textContent = jsonData[i][key];
				newParagraph.appendChild(text);
			}
			
		}

		listDiv.appendChild(newParagraph)

	}

}




