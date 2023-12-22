async function mapCoordinates(coordinates) {
	coordinates.forEach(coordinate => {
		var lat = coordinate.lat;
		var lon = coordinate.lon;
		L.marker([lat, lon]).addTo(map)
	})
}

async function sendRequest() {
	const City = document.getElementById('city').value;
	const Query = document.getElementById('query').value;

	const response = await fetch('/request', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ city: City, query: Query }),
	});

	const coordinates = await response.json();
	await mapCoordinates(coordinates);
}
