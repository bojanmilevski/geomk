const map = L.map('map').setView([41.6, 21.75], 9);

L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
	maxZoom: 19,
	attribution: '<a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>',
}).addTo(map);

const pinLayer = L.layerGroup().addTo(map);

const pinCoordinatesToMap = async (coordinates) => {
	pinLayer.clearLayers();

	if (coordinates.length == 0) {
		throw Error('No coordinates found for specified city');
	}

	coordinates.forEach(c => {
		L.marker([c.lat, c.lon]).addTo(pinLayer);
	});
}

const insertCoordinatesInListbox = async (city, coordinates) => {
	const listBox = document.getElementById('listBox');

	listBox.innerHTML = '';

	coordinates.forEach(c => {
		const option = document.createElement('option');
		option.value = c.id;
		option.textContent = `city: ${city}, lat: ${c.lat}, lon: ${c.lon}`;
		listBox.appendChild(option);
	})
}

const saveCoordinates = async () => {
	const listBox = document.getElementById('listBox');
	const ids = Array.from(listBox.selectedOptions, option => option.value);

	const response = await fetch('/api/save', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(ids),
	});

	if (!response.ok) {
		alert('save coordinates error');
		return;
	}

	for (let i = 0; i < listBox.options.length; i++) {
		listBox.options[i].selected = false;
	}
}

const sendRequest = async () => {
	const city = document.getElementById('city').value;
	const query = document.getElementById('query').value;

	const response = await fetch('/api/request', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ city, query }),
	});

	if (!response.ok) {
		alert('error message');
		return;
	}

	const json = await response.json();
	const coordinates = json.elements;

	try {
		await pinCoordinatesToMap(coordinates);
		await insertCoordinatesInListbox(city, coordinates);
	} catch (error) {
		alert(error.message);
		return;
	}
}
