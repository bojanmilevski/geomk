const mapCoordinates = async (coordinates) => {
	if (coordinates.length == 0) {
		throw Error('No coordinates found for specified city');
	}

	coordinates.forEach(coordinate => {
		var lat = coordinate.lat;
		var lon = coordinate.lon;
		L.marker([lat, lon]).addTo(map)
	})
}

const sendRequest = async () => {
	const City = document.getElementById('city').value;
	const Query = document.getElementById('query').value;

	const response = await fetch('/api/request', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ city: City, query: Query }),
	});

	const coordinates = await response.json();

	try {
		await mapCoordinates(coordinates);
	} catch (error) {
		alert(error);
		return;
	}
}
